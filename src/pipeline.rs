use crate::backend::Backend;

use crate::Res;
use crate::agent::Agent;

const PLANNER: &str = "You are a planner. Break the task down into at most three steps.";
const DEVELOPER: &str = "You are a developer. Write the code according to the plan.";
const REVIEWER: &str = "You are a code reviewer. Find bugs and suggest improvements in the code.";
const OUTPUT_REVIEWER: &str = "You are an output reviewer. Check whether the code fulfills the task.";

/// Runs the plan → code → review → output-check pipeline, printing each agent's result.
pub async fn run(backend: &Backend, task: &str, input: &str) -> Res<()> {
    let planner = Agent::new(backend, PLANNER);
    let developer = Agent::new(backend, DEVELOPER);
    let reviewer = Agent::new(backend, REVIEWER);
    let output_reviewer = Agent::new(backend, OUTPUT_REVIEWER);

    let plan = planner.run(task, input).await?;
    println!("\n\nPlanner agent:\n{plan}");

    let code = developer
        .run(task, &format!("{input}\n\nPlan:\n{plan}"))
        .await?;
    println!("\n\nDeveloper agent:\n{code}");

    let code_context = format!("{input}\n\nCode:\n{code}");

    let review = reviewer.run(task, &code_context).await?;
    println!("\n\nReviewer agent:\n{review}");

    let output = output_reviewer.run(task, &code_context).await?;
    println!("\n\nOutput reviewer agent:\n{output}");

    Ok(())
}
