use axum::http::{HeaderMap, HeaderValue};
use lazy_static::lazy_static;
use reqwest::{Client, Error, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

use crate::{data::config::CONFIG, pub_struct, utils::utils::fmt_num};

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PipelineTag {
    #[serde(rename = "text-classification")]
    TextClassification,
    #[serde(rename = "token-classification")]
    TokenClassification,
    #[serde(rename = "table-question-answering")]
    TableQuestionAnswering,
    #[serde(rename = "question-answering")]
    QuestionAnswering,
    #[serde(rename = "zero-shot-classification")]
    ZeroShotClassification,
    #[serde(rename = "translation")]
    Translation,
    #[serde(rename = "summarization")]
    Summarization,
    #[serde(rename = "feature-extraction")]
    FeatureExtraction,
    #[serde(rename = "text-generation")]
    TextGeneration,
    #[serde(rename = "fill-mask")]
    FillMask,
    #[serde(rename = "sentence-similarity")]
    SentenceSimilarity,
    #[serde(rename = "text-to-speech")]
    TexttoSpeech,
    #[serde(rename = "text-to-audio")]
    TexttoAudio,
    #[serde(rename = "automatic-speech-recognition")]
    AutomaticSpeechRecognition,
    #[serde(rename = "audio-to-audio")]
    AudiotoAudio,
    #[serde(rename = "audio-classification")]
    AudioClassification,
    #[serde(rename = "audio-text-to-text")]
    AudioTexttoText,
    #[serde(rename = "voice-activity-detection")]
    VoiceActivityDetection,
    #[serde(rename = "depth-estimation")]
    DepthEstimation,
    #[serde(rename = "image-classification")]
    ImageClassification,
    #[serde(rename = "object-detection")]
    ObjectDetection,
    #[serde(rename = "image-segmentation")]
    ImageSegmentation,
    #[serde(rename = "text-to-image")]
    TexttoImage,
    #[serde(rename = "image-to-text")]
    ImagetoText,
    #[serde(rename = "image-to-image")]
    ImagetoImage,
    #[serde(rename = "image-to-video")]
    ImagetoVideo,
    #[serde(rename = "unconditional-image-generation")]
    UnconditionalImageGeneration,
    #[serde(rename = "video-classification")]
    VideoClassification,
    #[serde(rename = "reinforcement-learning")]
    ReinforcementLearning,
    #[serde(rename = "robotics")]
    Robotics,
    #[serde(rename = "tabular-classification")]
    TabularClassification,
    #[serde(rename = "tabular-regression")]
    TabularRegression,
    #[serde(rename = "tabular-to-text")]
    TabulartoText,
    #[serde(rename = "table-to-text")]
    TabletoText,
    #[serde(rename = "multiple-choice")]
    MultipleChoice,
    #[serde(rename = "text-ranking")]
    TextRanking,
    #[serde(rename = "text-retrieval")]
    TextRetrieval,
    #[serde(rename = "time-series-forecasting")]
    TimeSeriesForecasting,
    #[serde(rename = "text-to-video")]
    TexttoVideo,
    #[serde(rename = "image-text-to-text")]
    ImageTexttoText,
    #[serde(rename = "image-text-to-image")]
    ImageTexttoImage,
    #[serde(rename = "image-text-to-video")]
    ImageTexttoVideo,
    #[serde(rename = "visual-question-answering")]
    VisualQuestionAnswering,
    #[serde(rename = "document-question-answering")]
    DocumentQuestionAnswering,
    #[serde(rename = "zero-shot-image-classification")]
    ZeroShotImageClassification,
    #[serde(rename = "graph-ml")]
    GraphMachineLearning,
    #[serde(rename = "mask-generation")]
    MaskGeneration,
    #[serde(rename = "zero-shot-object-detection")]
    ZeroShotObjectDetection,
    #[serde(rename = "text-to-3d")]
    Textto3D,
    #[serde(rename = "image-to-3d")]
    Imageto3D,
    #[serde(rename = "image-feature-extraction")]
    ImageFeatureExtraction,
    #[serde(rename = "video-text-to-text")]
    VideoTexttoText,
    #[serde(rename = "keypoint-detection")]
    KeypointDetection,
    #[serde(rename = "visual-document-retrieval")]
    VisualDocumentRetrieval,
    #[serde(rename = "any-to-any")]
    AnytoAny,
    #[serde(rename = "video-to-video")]
    VideotoVideo,
    #[serde(rename = "other")]
    Other,
}

impl PipelineTag {
    pub fn to_string(&self) -> String {
        match self {
            PipelineTag::TextClassification => "Text Classification".to_string(),
            PipelineTag::TokenClassification => "Token Classification".to_string(),
            PipelineTag::TableQuestionAnswering => "Table Question Answering".to_string(),
            PipelineTag::QuestionAnswering => "Question Answering".to_string(),
            PipelineTag::ZeroShotClassification => "Zero-Shot Classification".to_string(),
            PipelineTag::Translation => "Translation".to_string(),
            PipelineTag::Summarization => "Summarization".to_string(),
            PipelineTag::FeatureExtraction => "Feature Extraction".to_string(),
            PipelineTag::TextGeneration => "Text Generation".to_string(),
            PipelineTag::FillMask => "Fill-Mask".to_string(),
            PipelineTag::SentenceSimilarity => "Sentence Similarity".to_string(),
            PipelineTag::TexttoSpeech => "Text-to-Speech".to_string(),
            PipelineTag::TexttoAudio => "Text-to-Audio".to_string(),
            PipelineTag::AutomaticSpeechRecognition => "Automatic Speech Recognition".to_string(),
            PipelineTag::AudiotoAudio => "Audio-to-Audio".to_string(),
            PipelineTag::AudioClassification => "Audio Classification".to_string(),
            PipelineTag::AudioTexttoText => "Audio-Text-to-Text".to_string(),
            PipelineTag::VoiceActivityDetection => "Voice Activity Detection".to_string(),
            PipelineTag::DepthEstimation => "Depth Estimation".to_string(),
            PipelineTag::ImageClassification => "Image Classification".to_string(),
            PipelineTag::ObjectDetection => "Object Detection".to_string(),
            PipelineTag::ImageSegmentation => "Image Segmentation".to_string(),
            PipelineTag::TexttoImage => "Text-to-Image".to_string(),
            PipelineTag::ImagetoText => "Image-to-Text".to_string(),
            PipelineTag::ImagetoImage => "Image-to-Image".to_string(),
            PipelineTag::ImagetoVideo => "Image-to-Video".to_string(),
            PipelineTag::UnconditionalImageGeneration => {
                "Unconditional Image Generation".to_string()
            }
            PipelineTag::VideoClassification => "Video Classification".to_string(),
            PipelineTag::ReinforcementLearning => "Reinforcement Learning".to_string(),
            PipelineTag::Robotics => "Robotics".to_string(),
            PipelineTag::TabularClassification => "Tabular Classification".to_string(),
            PipelineTag::TabularRegression => "Tabular Regression".to_string(),
            PipelineTag::TabulartoText => "Tabular to Text".to_string(),
            PipelineTag::TabletoText => "Table to Text".to_string(),
            PipelineTag::MultipleChoice => "Multiple Choice".to_string(),
            PipelineTag::TextRanking => "Text Ranking".to_string(),
            PipelineTag::TextRetrieval => "Text Retrieval".to_string(),
            PipelineTag::TimeSeriesForecasting => "Time Series Forecasting".to_string(),
            PipelineTag::TexttoVideo => "Text-to-Video".to_string(),
            PipelineTag::ImageTexttoText => "Image-Text-to-Text".to_string(),
            PipelineTag::ImageTexttoImage => "Image-Text-to-Image".to_string(),
            PipelineTag::ImageTexttoVideo => "Image-Text-to-Video".to_string(),
            PipelineTag::VisualQuestionAnswering => "Visual Question Answering".to_string(),
            PipelineTag::DocumentQuestionAnswering => "Document Question Answering".to_string(),
            PipelineTag::ZeroShotImageClassification => {
                "Zero-Shot Image Classification".to_string()
            }
            PipelineTag::GraphMachineLearning => "Graph Machine Learning".to_string(),
            PipelineTag::MaskGeneration => "Mask Generation".to_string(),
            PipelineTag::ZeroShotObjectDetection => "Zero-Shot Object Detection".to_string(),
            PipelineTag::Textto3D => "Text-to-3D".to_string(),
            PipelineTag::Imageto3D => "Image-to-3D".to_string(),
            PipelineTag::ImageFeatureExtraction => "Image Feature Extraction".to_string(),
            PipelineTag::VideoTexttoText => "Video-Text-to-Text".to_string(),
            PipelineTag::KeypointDetection => "Keypoint Detection".to_string(),
            PipelineTag::VisualDocumentRetrieval => "Visual Document Retrieval".to_string(),
            PipelineTag::AnytoAny => "Any-to-Any".to_string(),
            PipelineTag::VideotoVideo => "Video-to-Video".to_string(),
            PipelineTag::Other => "Other".to_string(),
        }
    }
}

pub_struct! { CardData {
    license: Option<String>,
    tags: Option<Vec<String>>,
    task_categories: Option<Vec<String>>,
}}

pub_struct! { ModelConfig {
    model_type: Option<String>,
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseData {
    pub _id: String,
    pub id: String,
    pub private: bool,
    pub tags: Vec<String>,
    pub likes: u32,
    pub author: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub disabled: bool,
    #[serde(rename = "cardData")]
    pub card_data: CardData,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    #[serde(flatten)]
    pub base: BaseData,
    // without same useless fields
    pub pipeline_tag: Option<PipelineTag>,
    // e.g. transformers
    pub library_name: Option<String>,
    pub downloads: u32,
    #[serde(rename = "modelId")]
    pub model_id: String,
    pub config: Option<ModelConfig>,
}

// kube-xxxx/index.js
#[derive(Debug, Deserialize, Serialize)]
pub enum SpaceRuntimeStage {
    #[serde(rename = "NO_APP_FILE")]
    NoAppFile,
    #[serde(rename = "CONFIG_ERROR")]
    ConfigError,
    #[serde(rename = "BUILDING")]
    Building,
    #[serde(rename = "BUILD_ERROR")]
    BuildError,
    #[serde(rename = "APP_STARTING")]
    AppStarting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_BUILDING")]
    RunningBuilding,
    #[serde(rename = "RUNNING_APP_STARTING")]
    RunningAppStarting,
    #[serde(rename = "RUNTIME_ERROR")]
    RuntimeError,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "SLEEPING")]
    Sleeping,
}

// kube-xxxx/SpaceHardwareBadge.js
#[derive(Debug, Deserialize, Serialize)]
pub enum SpaceRuntimeHardware {
    #[serde(rename = "cpu-basic")]
    CpuBasic,
    #[serde(rename = "cpu-upgrade")]
    CpuUpgrade,
    #[serde(rename = "cpu-performance")]
    CpuPerfomance,
    #[serde(rename = "cpu-xl")]
    CpuXL,
    #[serde(rename = "a100-large")]
    A100Large,
    #[serde(rename = "zero-a10g")]
    ZeroA10G,
    #[serde(rename = "a10g-large")]
    A10GLarge,
    #[serde(rename = "a10g-largex2")]
    A10GLargeX2,
    #[serde(rename = "a10g-largex4")]
    A10GLargeX4,
    #[serde(rename = "a10g-small")]
    A10GSmall,
    #[serde(rename = "h100")]
    H100,
    #[serde(rename = "h100x8")]
    H100X8,
    #[serde(rename = "t4-medium")]
    T4Medium,
    #[serde(rename = "t4-small")]
    T4Small,
    #[serde(rename = "l4x1")]
    L4X1,
    #[serde(rename = "l4x4")]
    L4X4,
    #[serde(rename = "l40sx1")]
    L40SX1,
    #[serde(rename = "l40sx4")]
    L40SX4,
    #[serde(rename = "l40sx8")]
    L40SX8,
}

impl SpaceRuntimeHardware {
    pub fn to_string(&self) -> String {
        match self {
            SpaceRuntimeHardware::CpuUpgrade => "CPU Upgrade".to_string(),
            SpaceRuntimeHardware::CpuPerfomance => "CPU Performance".to_string(),
            SpaceRuntimeHardware::A100Large => "A100 Large".to_string(),
            SpaceRuntimeHardware::ZeroA10G => "Zero".to_string(),
            SpaceRuntimeHardware::A10GLarge => "A10G".to_string(),
            SpaceRuntimeHardware::A10GLargeX2 => "A10G".to_string(),
            SpaceRuntimeHardware::A10GLargeX4 => "A10G".to_string(),
            SpaceRuntimeHardware::A10GSmall => "A10G".to_string(),
            SpaceRuntimeHardware::H100 => "H100".to_string(),
            SpaceRuntimeHardware::H100X8 => "H100".to_string(),
            SpaceRuntimeHardware::T4Medium => "T4".to_string(),
            SpaceRuntimeHardware::T4Small => "T4".to_string(),
            SpaceRuntimeHardware::L4X1 => "L4".to_string(),
            SpaceRuntimeHardware::L4X4 => "L4".to_string(),
            SpaceRuntimeHardware::L40SX1 => "L40S".to_string(),
            SpaceRuntimeHardware::L40SX4 => "L40S".to_string(),
            SpaceRuntimeHardware::L40SX8 => "L40S".to_string(),
            _ => "".to_string(),
        }
    }
}

pub_struct! {
    SpaceRuntimeHardwareData {
        current: SpaceRuntimeHardware,
    }
}

pub_struct! {
    SpaceRuntime {
        stage: SpaceRuntimeStage,
        hardware: SpaceRuntimeHardwareData,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Space {
    #[serde(flatten)]
    pub base: BaseData,
    pub subdomain: String,
    pub host: String,
    // pub models: Option<Vec<String>>,
    pub runtime: SpaceRuntime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    #[serde(flatten)]
    pub base: BaseData,
    pub downloads: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ModelResponse {
    Failed(ErrorResponse),
    Valid(Model),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DatasetResponse {
    Failed(ErrorResponse),
    Valid(Dataset),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpaceResponse {
    Failed(ErrorResponse),
    Valid(Space),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RepoData {
    Model(Model),
    Dataset(Dataset),
    Space(Space),
}

impl RepoData {
    pub fn get_id(&self) -> String {
        match self {
            RepoData::Model(model) => model.base.id.clone(),
            RepoData::Dataset(dataset) => dataset.base.id.clone(),
            RepoData::Space(space) => space.base.id.clone(),
        }
    }

    pub fn get_likes(&self) -> String {
        let likes = match self {
            RepoData::Model(model) => model.base.likes,
            RepoData::Dataset(dataset) => dataset.base.likes,
            RepoData::Space(space) => space.base.likes,
        };

        fmt_num(likes as i32)
    }

    pub fn get_license(&self) -> Option<String> {
        match self {
            RepoData::Model(model) => model.base.card_data.license.clone(),
            RepoData::Dataset(dataset) => dataset.base.card_data.license.clone(),
            RepoData::Space(space) => space.base.card_data.license.clone(),
        }
    }

    pub fn get_downloads_count(&self) -> Option<String> {
        match self {
            RepoData::Model(model) => Some(fmt_num(model.downloads as i32)),
            RepoData::Dataset(dataset) => Some(fmt_num(dataset.downloads as i32)),
            RepoData::Space(_) => None,
        }
    }

    pub fn get_repo_tags(&self) -> Vec<String> {
        match self {
            RepoData::Model(model) => model.base.tags.clone(),
            RepoData::Dataset(dataset) => dataset.base.tags.clone(),
            RepoData::Space(space) => space.base.tags.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RepoResponse {
    Model(ModelResponse),
    Dataset(DatasetResponse),
    Space(SpaceResponse),
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    if !&CONFIG.huggingface_token.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&CONFIG.huggingface_token).unwrap(),
        );
    }

    headers
}

async fn request_intl<T: for<'de> serde::Deserialize<'de>>(pathname: &String) -> Result<T, Error> {
    let request_url = format!("https://huggingface.co/api/{pathname}");
    let headers = get_headers();
    let stats = REQ_CLIENT
        .get(&request_url)
        .headers(headers)
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(stats)
}

pub async fn get_model(username: &String, repo: &String) -> Result<ModelResponse, Error> {
    request_intl::<ModelResponse>(&format!("models/{username}/{repo}")).await
}

pub async fn get_dataset(username: &String, repo: &String) -> Result<DatasetResponse, Error> {
    request_intl::<DatasetResponse>(&format!("datasets/{username}/{repo}")).await
}

pub async fn get_space(username: &String, repo: &String) -> Result<SpaceResponse, Error> {
    request_intl::<SpaceResponse>(&format!("spaces/{username}/{repo}")).await
}
