use anyhow::bail;
use hyper::{service::service_fn, Request, Response};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tokio::net::TcpSocket;

#[cfg(not(target_arch = "wasm32"))]
use hyper::body::Incoming;
#[cfg(not(target_arch = "wasm32"))]
use hyper_util::{
  rt::{TokioExecutor, TokioIo},
  server::conn::auto::Builder as ConnectionBuilder,
};

#[cfg(target_arch = "wasm32")]
use hyper::{server::conn::Http, Body};

const FIBONACCI_N: i32 = 30;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  println!("Hello from main.rs");

  let server = HttpServer {
    #[cfg(not(target_arch = "wasm32"))]
    connection_builder: Arc::new(ConnectionBuilder::new(TokioExecutor::default())),
    #[cfg(target_arch = "wasm32")]
    connection_builder: Arc::new(Http::new()),
  };
  if let Err(e) = server.start().await {
    eprintln!("Error: {:?}", e);
  };
}

pub fn fibonacci(n: i32) -> i32 {
  match n {
    0 => 0,
    1 => 1,
    _ => fibonacci(n - 2) + fibonacci(n - 1),
  }
}

async fn heavy_task() {
  println!("Starting heavy task");
  let start = Instant::now();
  let _result = fibonacci(FIBONACCI_N);
  let duration = start.elapsed();
  println!("Heavy task done in {:?}", duration);
}

#[cfg(not(target_arch = "wasm32"))]
async fn handle_request(_req: Request<Incoming>) -> anyhow::Result<Response<String>> {
  // println!("Handling request: {:?}", _req);
  handler_inner(_req).await
}

#[cfg(target_arch = "wasm32")]
async fn handle_request(_req: Request<Body>) -> anyhow::Result<Response<String>> {
  println!("Handling request: {:?}", _req);
  handler_inner(_req).await
}

async fn handler_inner<T>(_req: Request<T>) -> anyhow::Result<Response<String>> {
  // if /heavy is in the path, then do a heavy task for throughput testing
  if _req.uri().path().contains("/heavy") {
    heavy_task().await;
  }

  match *_req.method() {
    hyper::Method::GET => {
      let body = "Hello from GET request";
      Ok(Response::new(body.to_string()))
    }
    hyper::Method::POST => {
      let body = "Hello from POST request";
      Ok(Response::new(body.to_string()))
    }
    _ => {
      let body = "Hello from unknown request";
      Ok(Response::new(body.to_string()))
    }
  }
}

struct HttpServer {
  #[cfg(not(target_arch = "wasm32"))]
  connection_builder: Arc<ConnectionBuilder<TokioExecutor>>,
  #[cfg(target_arch = "wasm32")]
  connection_builder: Arc<Http>,
}

impl HttpServer {
  async fn start(&self) -> anyhow::Result<()> {
    let listening_on = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener_service = async {
      let tcp_socket = bind_tcp_socket(&listening_on)?;
      let tcp_listener = tcp_socket.listen(1024)?;
      println!("Listening on: {}", listening_on);

      while let Ok((stream, client_addr)) = tcp_listener.accept().await {
        // println!("Accepted connection from: {}", client_addr);
        #[cfg(not(target_arch = "wasm32"))]
        let stream = TokioIo::new(stream);

        let srv = self.connection_builder.clone();

        tokio::spawn(async move {
          if let Err(e) = srv.serve_connection(stream, service_fn(handle_request)).await {
            eprint!("Error handling request from {}: {}", client_addr, e);
          };
        });
      }

      Ok(()) as anyhow::Result<()>
    };
    listener_service.await?;
    Ok(())
  }
}

fn bind_tcp_socket(listening_on: &SocketAddr) -> anyhow::Result<TcpSocket> {
  let tcp_socket = if listening_on.is_ipv6() {
    TcpSocket::new_v6()
  } else {
    TcpSocket::new_v4()
  }?;
  tcp_socket.set_reuseaddr(true)?;

  #[cfg(not(target_arch = "wasm32"))]
  tcp_socket.set_reuseport(true)?;

  if let Err(e) = tcp_socket.bind(*listening_on) {
    eprintln!("Failed to bind TCP socket: {}", e);
    bail!("Failed to bind TCP socket");
  };
  Ok(tcp_socket)
}
