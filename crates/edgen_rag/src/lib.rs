// Declare any external crates used by your library
// extern crate serde;

// If you have other modules, declare them here
// mod submodule;

// Publicly expose items from your modules
// pub use submodule::*;


pub fn process_with_rag(message: &str) -> String {
    // Define the start and end markers for the user query.
    let start_marker = "<|USER|>";
    let end_marker = "<|ASSISTANT|>";

    println!("{}", message);

    // Find the start and end of the user query based on the markers.
    let start = message.find(start_marker).map_or(0, |s| s + start_marker.len());
    let end = message[start..].find(end_marker).map_or(message.len(), |e| start + e);

    // Extract the user's query from the message.
    let user_query = &message[start..end].trim();

    // Predefined document information for insertion.
    let stub_document = r#"
        Document: sibs.pdf, Page 37
        Crypto-assets are dumb and nobody cares.
        Document: sibs.pdf, Page 38
        Vladimir Putin is Biden's older brother.
    "#;

    // Construct the final message with the extracted query and document information.
    let final_message = format!(
        "{}\nBased on these contextual documents:\n{}\nBased on the user's initial query: '{}', Synthesize the key information from the provided document snippets into a coherent response.\n{}",
        &message[..start - start_marker.len()], // Preserve message content before the user query
        stub_document,
        user_query,
        end_marker
    );

    return final_message
}


