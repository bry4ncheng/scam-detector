use axum::extract::FromRef;
use tracing::info;
use crate::domain::models::BotDistriktWebhookResponse;
use crate::error::AppResult;
use crate::repository::hugging_face_repo::HuggingFaceRepository;
use crate::service::service_registry::ServiceRegistry;
use crate::utils::process::process_scam;

#[derive(Clone)]
pub struct HuggingFaceService {
    hugging_face_repository: HuggingFaceRepository
}

impl FromRef<ServiceRegistry> for HuggingFaceService {
    fn from_ref(state: &ServiceRegistry) -> Self { state.hf_service.clone().unwrap() }
}


impl HuggingFaceService {
    pub fn new(
        hugging_face_repository: HuggingFaceRepository
    ) -> Self {
        Self { hugging_face_repository }
    }

    pub async fn get_phishing_scam(self, msg: String) -> AppResult<BotDistriktWebhookResponse> {

        let res_phish = self.clone()
            .hugging_face_repository
            .use_phishbot(msg.clone())
            .await?;

        let res_dc = self.clone()
            .hugging_face_repository
            .use_dc_scam_detector(msg.clone())
            .await?;
        let res_bert = self
            .hugging_face_repository
            .use_bert(msg)
            .await?;

        let vec = vec![res_phish, res_dc, res_bert];
        let (is_scam, confidence) = process_scam(vec);
        //Confidence of scam
        if is_scam {
            Ok(BotDistriktWebhookResponse {
                confidence,
                responses: vec![
                    "This is probably a scam! Please do not click on any links or reply to any messages.".to_string()
                ],
            })
        } else {
            Ok(BotDistriktWebhookResponse {
                confidence,
                responses: vec![
                    "This is unlikely a scam!".to_string()
                ],
            })
        }
    }
}