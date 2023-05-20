# Alternative Sentences
Given a sentence, replace each word with a synonym obtained by querying Onelook's reverse dictionary API.

## Usage 
After downloading the repository, run the program with `cargo run -- --sentence "{YOUR SENTENCE}"` (substituting in your text without the curly braces).

## Examples
| Input | Output |
|-------|--------|
| "norway has the best prisons" | "norge hour angle gazette top-grade prison house" |
| "the api i'm using isn't great" | "gazette genus apis i am exploitation is not large" |

## Notice
The Datamuse API seems to send over incomplete JSON data — either that or I'm misinterpreting the data somehow — and so the program is unlikely to work for sentences containing the most common words.