|    | required | default_value | input  | result | present | count |  values   |
|----|:--------:|:-------------:|:------:|:------:|:-------:|:-----:|:---------:|
|    |   `i`    |      `i`      |  `i`   |  `o`   |   `o`   |  `o`  |    `o`    |
| 1  |  false   |     None      |   []   |   ok   |  false  |   0   |    []     |
| 2  |  false   |     None      |   X    |   ok   |  true   |   1   | [Some(X)] |
| 3  |  false   |    Some(A)    |   []   |   ok   |  false  |   0   | [Some(A)] |
| 4  |  false   |    Some(A)    |   X    |   ok   |  true   |   1   | [Some(X)] |
| 5  |   true   |     None      |   []   | error  |         |       |           |
| 6  |   true   |     None      |   X    |   ok   |  true   |   1   | [Some(X)] |
| 7  |   true   |    Some(A)    |   []   |   ok   |  false  |   0   | [Some(A)] |
| 8  |   true   |    Some(A)    |   X    |   ok   |  true   |   1   | [Some(X)] |
| 9  |    -     |       -       |  X Y   | error  |         |       |           |
| 10 |    -     |       -       |   -h   | error  |         |       |           |
| 11 |  false   |     None      | --help | error  |         |       |           |
| 12 |  false   |     None      |   --   | error  |         |       |           |
| 13 |  false   |     None      |  -- a  | error  |         |       |           |
