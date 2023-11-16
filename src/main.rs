#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::form::Form;
use rocket::response::content::RawHtml;


#[derive(FromForm)]
struct Exercise {
    exercise: String,
    weight: String,
}

#[post("/submit-exercise", data = "<exercise_form>")]
async fn submit_exercise(exercise_form: Form<Exercise>) -> RawHtml<String> {
    let exercise = exercise_form.into_inner();
    RawHtml(format!("<p>Exercise: {} - Weight: {}</p>", exercise.exercise, exercise.weight))
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit_exercise])
        .mount("/static", FileServer::from("static"))
}