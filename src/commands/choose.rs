use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;

use rand::seq::SliceRandom;

#[command]
#[aliases(pick, decide)]
#[description("Ask the bot to decide for you between some options")]
#[usage("<one option>? or <multiple choices>")]
#[example("pineapple banana apple lemon\n\nI choose apple!")]
#[help_available]
#[bucket("basic")]
async fn choose (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    if args.len() == 0 {
        msg.reply(ctx, "This command needs at least one argument").await?;
    } else if args.len() == 1 {
        msg.reply(ctx, ask_yes_or_no()).await?;
    } else {
        msg.reply(ctx, ask_multiple_choice(&mut args)).await?;
    }
    Ok(())
}

#[command]
#[description("Ask the bot some yes/no question")]
#[usage("<question to ask>")]
#[example("Should I become unreazonable\n\nAbsolutely!")]
#[help_available]
#[bucket("basic")]
async fn ask (ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() == 0 {
        msg.reply(ctx, "There's no question to ask!").await?;
    } else {
        msg.reply(ctx, ask_yes_or_no()).await?;
    }
    Ok(())
}

fn ask_yes_or_no() -> String {

    let answers = [
        format!("Definitively!"),
        format!("Maybe another day"),
        format!("I don't think it's a good idea..."),
        format!("Sure thing"),
        format!("Ehhhhhh"),
        format!("I would do it 100%"),
        format!("How is that even a question?!? Of course!"),
        format!("I don't think I should be answering such thing..."),
        format!("ERROR: Your free trial to my knowledge has expired, try another time"),
        format!("How am I supposed to know?"),
        format!("Why not"),
        format!("why."),
        format!("Surely nothing can go bad, so go ahead"),
        format!("I will have to say no this time"),
        format!("Sounds like a very good idea :D"),
        format!("Sounds like a good idea"),
        format!("Sounds like a terrible idea"),
        format!("Sounds like an Ariri's idea D:"),
        format!("no."),
        format!("-. ---"),
        format!("... .."),
        format!("Never!"),
        format!(":x"),
        format!("Not today, maybe tomorrow!"),
        format!("If it was yesterday sure, but not anymore"),
        format!("Negative"),
        format!("Positive"),
        format!("Uhhhhhhhhhhh *crashes*"),
        format!("Go for it chief"),
        format!("Over my dead body"),
        format!("Over my cold and dead hands \n ... \n I'm a robot so ... yeah"),
        format!("Roll a D20!"),
        format!("Flip a coin"),
        format!("slay gurl"),
        format!("Do I have to say it? Hell no!"),
        format!("you silly, of course!"),
        format!("The answer is in your heart"),
        format!("For your and my safety I'll have to say no"),
        format!("Always!"),
        format!("haha no :)."),
    ];

    answers.choose(&mut rand::thread_rng()).unwrap().to_string()
}
fn ask_multiple_choice (args: &mut Args) -> String {
    let mut options: Vec<String>  = vec![];
    for _ in 0..args.len() {
        options.push(args.single::<String>().unwrap());
    };
    let choice = options.choose(
        &mut rand::thread_rng()).unwrap().to_string();
    let sec_choice = options.choose(
        &mut rand::thread_rng()).unwrap().to_string();

    let answers = [
        format!("{choice} 100%"),
        format!("I think I'll go with {choice} today"),
        format!("Mhm, hard to decide ... but I think {choice}"),
        format!("I'd kill for {choice} :)"),
        format!("{choice} seems like the right choice"),
        format!("Everything feels kinda the same\n {choice} I guess"),
        format!("{choice} seems like the reazonable choice, so go for {sec_choice}"),
        format!("{sec_choice} is better"),
        format!("Do {sec_choice} and your subscription to life will be abruptly ended"),
        format!("{choice} over {sec_choice} 100%"),
        format!("{choice} today, {sec_choice} tomorrow"),
        format!("{choice}!!"),
        format!("Unless {sec_choice} is time-limited, go for {choice}"),
        format!("{sec_choice} is definitively a bad idea"),
        format!("I'm not so sure but {choice} looks better"),
        format!("Nothing wrong can go if you choose {choice}"),
        format!("{sec_choice} is a hoax"),
        format!("Do {choice}, live for {sec_choice}"),
        format!("Long live {choice}!"),
        format!("{choice} is what a tyrant would do"),
        format!("Add spice to {choice} and go for it"),
        format!("{choice} is the answer to life, the universe, and everything"),
        format!("Nothing! Do a backflip instead"),
        format!("{choice} if you want it easy. {sec_choice} if you want it spicy"),
        format!("{choice} is barely better than {sec_choice}"),
        format!("{choice} uwu"),
        format!("{sec_choice} only if you can {choice} afterwards, if not then just {choice}"),
        format!("{choice} sounds fun!"),
        format!("{choice} if morning, {sec_choice} otherwise"),
        format!("{sec_choice} is disgusting, ew"),
        format!("If {choice} has a million fans I am one of them. If {choice} has ten fans I am one of them. If {choice} has no fans, that means I am no more on the earth. If the World is against {choice}, I am against the World. I love {choice} till my last breath."),
        format!("{choice}, dummy!"),
        format!("{choice} is yummy"),
        format!("Eat, {choice}, sleep"),
        format!("It's elementary my dear Watson, {choice} is the obvious choice! Quite a simple deduction, try to keep up"),
        format!("The voices answer with {choice}"),
        format!("Think smarter not harder, choose {choice}"),
        format!("{choice} deserves an opportunity"),
        format!("{choice} was revealed to me in a dream"),
        format!("{choice} sounds like something PK'd do, choose the opposite"),
    ];
    answers.choose(&mut rand::thread_rng()).unwrap().to_string()
}