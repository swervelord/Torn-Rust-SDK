use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use torn_sdk_planner::{HttpTransport, TransportError, TransportRequest, TransportResponse};

#[derive(Debug)]
pub struct MockTransportState {
    responses: Mutex<VecDeque<Result<TransportResponse, TransportError>>>,
    requests: Mutex<Vec<TransportRequest>>,
}

#[derive(Debug, Clone)]
pub struct MockTransport {
    state: Arc<MockTransportState>,
}

impl MockTransport {
    pub fn with_responses(responses: Vec<Result<TransportResponse, TransportError>>) -> Self {
        Self {
            state: Arc::new(MockTransportState {
                responses: Mutex::new(responses.into()),
                requests: Mutex::new(Vec::new()),
            }),
        }
    }

    pub fn requests(&self) -> Vec<TransportRequest> {
        self.state
            .requests
            .lock()
            .expect("requests lock should not be poisoned")
            .clone()
    }
}

impl HttpTransport for MockTransport {
    async fn execute(
        &self,
        request: &TransportRequest,
    ) -> Result<TransportResponse, TransportError> {
        self.state
            .requests
            .lock()
            .expect("requests lock should not be poisoned")
            .push(request.clone());
        self.state
            .responses
            .lock()
            .expect("responses lock should not be poisoned")
            .pop_front()
            .expect("missing mocked response")
    }
}
