use std::{
    convert::Infallible,
    fmt::Display,
    future::{ready, Ready},
    task::Poll,
};

use tower::Service;

#[derive(Debug, PartialEq)]
pub struct EchoRequest(String);

impl Display for EchoRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Anything that can be turned into a String can be an EchoRequest
impl<T> From<T> for EchoRequest
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        EchoRequest(value.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct EchoResponse(String);

impl Display for EchoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Anything that can be turned into a String can be an EchoRequest
impl<T> From<T> for EchoResponse
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        EchoResponse(value.into())
    }
}

#[derive(Debug)]
pub struct EchoService;

impl Service<EchoRequest> for EchoService {
    type Response = EchoResponse;

    type Error = Infallible;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: EchoRequest) -> Self::Future {
        ready(Ok(EchoResponse(req.0)))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::task;

    use futures::task::noop_waker_ref;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_poll_ready() {
        let mut echo_service = EchoService;
        let res = echo_service.poll_ready(&mut task::Context::from_waker(noop_waker_ref()));
        assert!(res.is_ready());

        if let Poll::Ready(result) = res {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_call() {
        let mut echo_service = EchoService;
        let res = echo_service.call(EchoRequest(format!("Tower test"))).await;

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, EchoResponse(format!("Tower test")));
    }

    #[tokio::test]
    async fn test_usage() {
        // oneshot
        let service = EchoService;
        let response = service
            .oneshot(EchoRequest(format!("Tower test")))
            .await
            .unwrap();

        println!("{response}");
        assert_eq!(response, EchoResponse(format!("Tower test")));

        // ready
        let mut service = EchoService;
        let service = service.ready().await.unwrap();

        let response = service
            .call(EchoRequest(format!("Tower test2")))
            .await
            .unwrap();

        println!("{response}");
        assert_eq!(response, EchoResponse(format!("Tower test2")));

        // ready_oneshot
        let mut service = EchoService;
        let mut service = service.ready_oneshot().await.unwrap();

        let response = service
            .call(EchoRequest(format!("Tower test3")))
            .await
            .unwrap();

        println!("{response}");
        assert_eq!(response, EchoResponse(format!("Tower test3")));
    }
}
