use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::conversation::{Conversation, Message};
use crate::api::converse;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let(conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        // Actix web connection call (auto gens the server code call)
        converse(cx, conversation.get())
    });

    // Every time the send action is called, this effect will run
    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };
            
            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    // Effect to handle when the server call returns
    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.pop();
                c.messages.push(Message {
                    text: response,
                    user: false,
                });
            });
        };
    });
    
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rust LLM"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}
