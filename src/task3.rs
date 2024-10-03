#[test]

/*
// Fix the errors
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@10 | 11 | 12 } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
*/

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id } if (3..=7).contains(&id) => println!("Found an id in range [3, 7]: {}", id), // Теперь id захвачен
        Message::Hello { id: newid @ (10 | 11 | 12) } => { // Захват id с объединением
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum Message {
    Hello { id: i32 },
}

/*
Использование guard (охранника): Вместо попытки сопоставить id с диапазоном,
мы используем условие if (3..=7).contains(&id) для проверки, находится ли id в этом диапазоне.
Теперь id может быть использован в println!.

match: Используется правильный синтаксис захвата переменной newid с
использованием @ и правильно заключаем значения 10 | 11 | 12 в круглые скобки.
*/