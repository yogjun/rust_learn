#[derive(Debug)]
enum Gender {
    Male = 0,
    Female = 1,
    Else = 2,
}#[derive(Debug, Copy, Clone)]
struct UserId(u64);
#[derive(Debug, Copy, Clone)]
struct TopicId(u64);
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let jun = User {
        id: UserId(1),
        name: "jun".into(),
        gender: Gender::Male,
    };

    let jia = User {
        id: UserId(2),
        name: "jia".into(),
        gender: Gender::Female,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "mytopic".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((jun.id, topic.id));
    let event2 = Event::Join((jia.id, topic.id));
    // let event3 = Event::Message((jun.id, topic.id, "my first message".into()));

    let event3 = Event::Message((jun.id, topic.id, "Hello world!".into()));
    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
}
