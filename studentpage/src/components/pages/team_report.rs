use yew::prelude::*;

use crate::components::atoms::text_area::{TextArea, TextAreaVariant};
use crate::components::molecules::team_form::TeamForm;
use crate::components::atoms::instructions::Instructions;
use crate::components::atoms::range::Range;

#[function_component(TeamReport)]
pub fn team_report() -> Html {

    let range_state = use_state(|| "??".to_owned());
    let cloned_range_state = range_state.clone();
    let range_changed = Callback::from(move |value: String| {
        cloned_range_state.set(value);
    });

    html! {
        <TeamForm>
            <Instructions>
                <p> { "Consider the following four pairs of questions hierarchically. They are not the same question. If you think they are, then you are likely not using an appropriate breadth and depth of software-engineering thought. This course is a practical application of the aspects of product, process, and people. We are trying to account for everything: not just to create a good product, but also to learn from the process to improve the people. Reflect on the experience of the entire team collectively over this sprint. You do not need to account for all activities, just two that were representative of easiest and hardest. Use activity codes (e.g., A1) for specific references, but most of the response should be in sentence form."}        
                </p>
                <p> { "For reference, understand relates to the comprehension of what need to be done; approach to how you think it should be solved; solve to implementing the actual solution; and evalutate to demonstrating to yourself and your team (if applicable) that the performance of your solution is consistent with everything else in the project. Remember The Cartoon from CS 350."}
                </p>
            </Instructions>
            <TextArea
                label="Which aspects of the current work are the easiest to understand?"
                id="understand-easy"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the hardest to understand?"
                id="understand-hard"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the easiest to approach?"
                id="approach-easy"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the hardest to approach?"
                id="approach-hard"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the easiest to solve?"
                id="solve-easy"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the hardest to solve?"
                id="solve-hard"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the easiest to evaluate?"
                id="evaluate-easy"
                variant={ TextAreaVariant::Split }/>
            <TextArea
                label="Which aspects of the current work are the hardest to evaluate?"
                id="evaluate-hard"
                variant={ TextAreaVariant::Split }/>
            <Range 
                label={format!("How far along (as a percent) do you feel you are toward the final goal? %{}", &*range_state)}
                id="completion-percent"
                handle_onchange={range_changed}/>
            <TextArea 
                label="Does this pace seem likely to succeed?"
                id="pace-succeed"
                rows={"2"}
                variant={ TextAreaVariant::Wide }/>
            <TextArea
                label="Did you meet with your client this week? If not, when was the last time?"
                id="client-meeting"
                variant={ TextAreaVariant::Wide }/>
            <TextArea
                label="Are there any issues, concern, or comments about the project?"
                id="issues-concerns-comments"
                variant={ TextAreaVariant::Wide }/>
        </TeamForm>
    }
}