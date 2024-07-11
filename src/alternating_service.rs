use std::{
    convert::Infallible,
    fmt::Display,
    future::{ready, Ready},
    task::Poll,
};

use tower::Service;

#[derive(Debug, PartialEq)]
pub struct AlternatingRequest;

#[derive(Debug, PartialEq)]
pub struct AlternatingResponse;

#[derive(Debug, Default)]
pub struct AlternatingService {
    ready_state: bool,
}

impl Service<AlternatingRequest> for AlternatingService {
    type Response = AlternatingResponse;

    type Error = Infallible;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.ready_state {
            cx.waker().wake_by_ref();
            self.ready_state = false;
            Poll::Pending
        } else {
            self.ready_state = true;
            Poll::Ready(Ok(()))
        }
    }

    fn call(&mut self, req: AlternatingRequest) -> Self::Future {
        if !self.ready_state {
            panic!("service not ready. poll_ready must be called first");
        }
        self.ready_state = false;
        ready(Ok(AlternatingResponse))
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
        let mut service = AlternatingService::default();

        let res = service.poll_ready(&mut std::task::Context::from_waker(noop_waker_ref()));
        assert!(res.is_ready());

        let res = service.poll_ready(&mut std::task::Context::from_waker(noop_waker_ref()));
        assert!(res.is_pending());
    }

    #[tokio::test]
    async fn test_usage() {
        //oneshot
        let mut service = AlternatingService::default();
        service.ready_state = true;

        // the unwrap ensures that poll_ready is called until it is ready because the first call fails
        let _res = service.oneshot(AlternatingRequest).await.unwrap();

        //oneshot
        let mut service = AlternatingService::default();
        service.ready_state = true;

        // the unwrap ensures that poll_ready is called until it is ready because the first call fails
        let service = service.ready().await.unwrap();
        let _response = service.call(AlternatingRequest).await.unwrap();

        //ready_oneshot
        let mut service = AlternatingService::default();
        service.ready_state = true;

        // the unwrap ensures that poll_ready is called until it is ready because the first call fails
        let mut service = service.ready_oneshot().await.unwrap();
        let _response = service.call(AlternatingRequest).await.unwrap();
    }
}
