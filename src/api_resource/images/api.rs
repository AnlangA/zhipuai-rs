///! the images AI api of zhipu
use super::data::*;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/images/generations";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct ImagesApiRequest {
    /// model name
    model: String,
    /// prompt text
    prompt: String,
    /// size of image
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<ImageSize>,
    /// user id
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
}

impl ImagesApiRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct ImagesApiRequestBuilder {
    /// model name
    model: String,
    /// prompt text
    prompt: String,
    /// size of image
    size: Option<ImageSize>,
    /// user id
    user_id: Option<String>,
}

impl ImagesApiRequestBuilder {
    // Public function `new` that creates a new instance of the struct.
    // It takes a string reference `model` as a parameter and returns an instance of `Self`.
    pub fn new(model: &str) -> Self {
        // Create a new instance of `Self` (the struct this function is defined in).
        // Initialize the struct fields as follows:
        Self {
            // Set the `model` field to the value of `model` converted to a `String`.
            model: model.to_string(),
            // Initialize the `prompt` field as an empty `String`.
            prompt: String::new(),
            // Initialize the `size` field as `None`, indicating no size is set initially.
            size: None,
            // Initialize the `user_id` field as `None`, indicating no user ID is set initially.
            user_id: None,
        }
    }
    // Define a public method named `prompt` which takes a mutable reference to `self` and a string slice `prompt` as arguments.
    // This method is intended to set a prompt message for some functionality, likely in a command-line interface or similar context.
    pub fn prompt(mut self, prompt: &str) -> Self {
        // Assign the value of the input string slice `prompt` to the `prompt` field of `self` after converting it to a `String` type.
        // This conversion is necessary because `prompt` is currently a string slice (`&str`), and we likely need an owned `String` for storage.
        self.prompt = prompt.to_string();
        // Return the modified instance of `self` to allow for method chaining.
        // This allows the caller to set the prompt and then immediately call another method on the same instance.
        self
    }
    // Define a public method named `size` for a struct (not shown in the snippet).
    // This method takes a mutable reference to `self` and an `ImageSize` parameter.
    // The method returns a mutable reference to the same struct instance (`Self`).
    pub fn size(mut self, size: ImageSize) -> Self {
        // Assign the provided `ImageSize` value to the `size` field of the struct.
        // This is done by wrapping the `size` in a `Some` variant, indicating that the field is now set.
        self.size = Some(size);
        // Return the modified struct instance.
        // This allows method chaining, where the caller can call another method on the returned instance.
        self
    }
    // Define a public method named `user_id` for a struct (not shown in the snippet).
    // This method takes a mutable reference to `self` and a string slice (`&str`) as parameters.
    pub fn user_id(mut self, user_id: &str) -> Self {
        // Assign the provided `user_id` to the struct's `user_id` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.user_id = Some(user_id.to_string());
        // Return the modified instance of the struct.
        self
    }
    // Define a public function named `build` that consumes the instance (`self`) of the struct it's defined in.
    pub fn build(self) -> (String, ImagesApiRequest) {
        // Return a tuple containing two elements:
        // 1. A new `String` created from the constant `API_URL`.
        // 2. The current instance (`self`) of the struct, cast to `ImagesApiRequest`.
        (
            API_URL.to_string(),
            ImagesApiRequest {
                model: self.model,
                prompt: self.prompt,
                size: self.size,
                user_id: self.user_id,
            },
        )
    }
}
