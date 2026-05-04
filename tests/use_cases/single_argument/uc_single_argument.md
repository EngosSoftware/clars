|   | required | default_value | provided | result | present | count |  values   |
|---|:--------:|:-------------:|:--------:|:------:|:-------:|:-----:|:---------:|
|   |   `i`    |      `i`      |   `i`    |  `o`   |   `o`   |  `o`  |    `o`    |
| 1 |  false   |     None      |  false   |   ok   |  false  |   0   |    []     |
| 2 |  false   |     None      |   true   |   ok   |  true   |   1   | [Some(X)] |
| 3 |  false   |    Some(A)    |  false   |   ok   |  false  |   0   | [Some(A)] |
| 4 |  false   |    Some(A)    |   true   |   ok   |  true   |   1   | [Some(X)] |
| 5 |   true   |     None      |  false   | error  |         |       |           |
| 6 |   true   |     None      |   true   |   ok   |  true   |   1   | [Some(X)] |
| 7 |   true   |    Some(A)    |  false   |   ok   |  false  |   0   | [Some(A)] |
| 8 |   true   |    Some(A)    |   true   |   ok   |  true   |   1   | [Some(X)] |
