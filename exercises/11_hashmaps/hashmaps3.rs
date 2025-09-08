// Надано список рахунків (один на рядок) футбольного матчу. Кожен рядок має форму
// "<назва_команди_1>,<назва_команди_2>,<голи_команди_1>,<голи_команди_2>"
// Приклад: "Англія,Франція,4,2" (Англія забила 4 голи, Франція 2).
//
// Ви повинні побудувати таблицю рахунків, що містить назву команди, загальну
// кількість голів, які забила команда, і загальну кількість голів,
// які пропустила команда.

use std::collections::HashMap;

// Структура для зберігання деталей голів команди.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // Назва команди є ключем, а пов’язана структура - значенням.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // ПРИМІТКА: Ми використовуємо `unwrap`, бо ми ще не вивчали обробку помилок.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Заповніть таблицю рахунків витягнутими деталями.
        // Пам’ятайте, що голи, забиті командою 1, будуть кількістю голів,
        // пропущених командою 2. Аналогічно, голи, забиті командою 2, будуть
        // кількістю голів, пропущених командою 1.
    }

    scores
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "Англія,Франція,4,2
Франція,Італія,3,1
Польща,Іспанія,2,0
Німеччина,Англія,2,1
Англія,Іспанія,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["Англія", "Франція", "Німеччина", "Італія", "Польща", "Іспанія"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Англія").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Іспанія").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
