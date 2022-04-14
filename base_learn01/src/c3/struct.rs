// enum Gender {
//     Male = 0,
//     Female = 1,
//     Else = 2,
// }

// struct UserId(u64);

// struct TopicId(u64);

// struct User {
//     userId: UserId,
//     name: String,
//     Gender: Gender,
// }

// struct Topic {
//     topicId: TopicId,
//     name: String,
//     owner: UserId,
// }

// enum Event {
//     Join(User, Topic),
//     Leave(User, Topic),
//     Message(User, Topic, String),
// }

// fn demo() {
//     let jun = User(UserId(1), "jun", Gender(0));
//     let jia = User(UserId(2), "jia", Gender(1));

//     let topic = Topic(TopicId(111), "lunch", jun.userId);

//     let event1 = Event::Join(jun, topic);
//     let event2 = Event::leave(jun, topic);
//     let event1 = Event::Message(jun, topic, "my first message");
//     println!(
//         "event1: {:?}, event2: {:?}, event3: {:?}",
//         event1, event2, event3
//     );
// }
