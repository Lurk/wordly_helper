# [Wordle](https://www.powerlanguage.co.uk/wordle/) game cheat api


the dictionary is
from [guess_my_word repo](https://github.com/powerlanguage/guess-my-word/blob/master/wordlist/sowpods.txt)


intended to be a showcase how awesome [a_thing](https://github.com/Lurk/a_thing) is

## API

### [POST] /api/word

json body

```rust 
struct Rules {
  contains: String,
  not_contains: String,
  positional_contains: String,
  positional_not_contains: Vec<String>,
}
```
where:
* positional_contains is string with unknown letters marked as "_"
* positional_not_contains is array of strings where unknown letters marked as "_"


```json
{
  "contains": "eol",
  "not_contains": "ars",
  "positional_contains": "_e__o",
  "positional_not_contains": [
    "___o_",
    "l____"
  ]
}
```
