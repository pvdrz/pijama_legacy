window.BENCHMARK_DATA = {
  "lastUpdate": 1597014831957,
  "repoUrl": "https://github.com/christianpoveda/pijama",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "59b6d26e0be8a2e056d6826b9ce39366c5cc4fdc",
          "message": "Update bench.yml",
          "timestamp": "2020-05-21T11:28:04-05:00",
          "tree_id": "b4a645c0353367978763b71f809696b9391aaf4f",
          "url": "https://github.com/christianpoveda/pijama/commit/59b6d26e0be8a2e056d6826b9ce39366c5cc4fdc"
        },
        "date": 1590080218824,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26579,
            "range": "± 744",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 54289,
            "range": "± 811",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 70565,
            "range": "± 4563",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 121706,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 540592,
            "range": "± 7852",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 102751,
            "range": "± 1984",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 827841,
            "range": "± 16240",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 422052,
            "range": "± 14382",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 163754,
            "range": "± 1951",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 176428,
            "range": "± 3674",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1870,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1571,
            "range": "± 50",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "git@christianpoveda.xyz",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "git@christianpoveda.xyz",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "distinct": true,
          "id": "86c013655ce0356bf78c7e8967aaca331e07e358",
          "message": "Add workflow for benchmarking",
          "timestamp": "2020-05-21T12:08:10-05:00",
          "tree_id": "b4a645c0353367978763b71f809696b9391aaf4f",
          "url": "https://github.com/christianpoveda/pijama/commit/86c013655ce0356bf78c7e8967aaca331e07e358"
        },
        "date": 1590081352327,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 25913,
            "range": "± 1231",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 54707,
            "range": "± 4048",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 68281,
            "range": "± 4104",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 116926,
            "range": "± 5732",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 530777,
            "range": "± 29419",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 101190,
            "range": "± 5923",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 833678,
            "range": "± 47970",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 420599,
            "range": "± 22132",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 165847,
            "range": "± 11475",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 180880,
            "range": "± 12575",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1827,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1548,
            "range": "± 65",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "829aa8131b4358d6f74b3592f2bc52b9becfa172",
          "message": "Merge pull request #67 from DarkDrek/parse-error-context\n\nAdd context to some errors during parsing",
          "timestamp": "2020-05-21T12:16:05-05:00",
          "tree_id": "c183a461f690b1f13fbe39632b5b377fc04554c7",
          "url": "https://github.com/christianpoveda/pijama/commit/829aa8131b4358d6f74b3592f2bc52b9becfa172"
        },
        "date": 1590081748413,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 28144,
            "range": "± 1725",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 56440,
            "range": "± 4877",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 69695,
            "range": "± 5165",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 120388,
            "range": "± 6337",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 537711,
            "range": "± 25188",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 103700,
            "range": "± 5662",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 843877,
            "range": "± 46049",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 447966,
            "range": "± 35651",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 169461,
            "range": "± 11857",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 183823,
            "range": "± 15436",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1881,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1639,
            "range": "± 113",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "22160c64d7a153341585701eb95551c323d3638e",
          "message": "Merge pull request #65 from christianpoveda/banish-rec\n\nRemove `rec` keyword",
          "timestamp": "2020-05-21T12:31:30-05:00",
          "tree_id": "b1e0ae7387746ce487ebc9e297a457a60eac467b",
          "url": "https://github.com/christianpoveda/pijama/commit/22160c64d7a153341585701eb95551c323d3638e"
        },
        "date": 1590082634363,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26028,
            "range": "± 858",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 52738,
            "range": "± 1899",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 67645,
            "range": "± 3756",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 115089,
            "range": "± 4988",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 527730,
            "range": "± 18640",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 102309,
            "range": "± 4132",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 798000,
            "range": "± 25355",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 408438,
            "range": "± 15083",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 161478,
            "range": "± 5110",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 168354,
            "range": "± 7557",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1866,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1533,
            "range": "± 56",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5417f4f6d90c406786e0fd5c987c8a8f30187e07",
          "message": "Merge pull request #70 from christianpoveda/prim-ast\n\nAdd new Primitive element to the AST",
          "timestamp": "2020-05-21T14:32:42-05:00",
          "tree_id": "51d6d23259978adb20e7966cab1e1241177bcc98",
          "url": "https://github.com/christianpoveda/pijama/commit/5417f4f6d90c406786e0fd5c987c8a8f30187e07"
        },
        "date": 1590089955091,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 27965,
            "range": "± 761",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 58472,
            "range": "± 2075",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 73261,
            "range": "± 5217",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 128691,
            "range": "± 10420",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 563810,
            "range": "± 52607",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 107226,
            "range": "± 4384",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 898170,
            "range": "± 32427",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 436248,
            "range": "± 11386",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 174014,
            "range": "± 6374",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 188472,
            "range": "± 7232",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1810,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1539,
            "range": "± 56",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b084be309a463b3d7bad6d6dcdfb785742ad06f",
          "message": "Merge pull request #72 from DarkDrek/primitive-print\n\nAdd the primitive print",
          "timestamp": "2020-05-21T20:08:04-05:00",
          "tree_id": "42b6381df3c3e5c16be59a55fd02f0dfae5d7a77",
          "url": "https://github.com/christianpoveda/pijama/commit/2b084be309a463b3d7bad6d6dcdfb785742ad06f"
        },
        "date": 1590110070776,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 28244,
            "range": "± 2259",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 59490,
            "range": "± 2098",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 73725,
            "range": "± 1938",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 124921,
            "range": "± 7011",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 567210,
            "range": "± 16653",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 111021,
            "range": "± 6467",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 907713,
            "range": "± 61520",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 446509,
            "range": "± 13119",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 180486,
            "range": "± 4903",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 197206,
            "range": "± 10268",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 2014,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1640,
            "range": "± 49",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "50bd57a20e87f9b664574d4fbf0cd0527b16b39b",
          "message": "Merge pull request #75 from christianpoveda/refactor-ty-check\n\nRefactor and document the `ty` module",
          "timestamp": "2020-05-22T18:52:48-05:00",
          "tree_id": "dece64638bd342645480fa7eab88d455a90fd7cb",
          "url": "https://github.com/christianpoveda/pijama/commit/50bd57a20e87f9b664574d4fbf0cd0527b16b39b"
        },
        "date": 1590191956568,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 27112,
            "range": "± 1535",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 53028,
            "range": "± 2826",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 69959,
            "range": "± 5202",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 118794,
            "range": "± 6593",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 539336,
            "range": "± 24194",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 104056,
            "range": "± 6352",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 832705,
            "range": "± 32303",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 422325,
            "range": "± 21868",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 167271,
            "range": "± 7696",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 181999,
            "range": "± 8123",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1912,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1518,
            "range": "± 84",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "985c750b7ae5851db496ece9e803e53ac6a8089d",
          "message": "Merge pull request #74 from seanchen1991/elif\n\nmultibranch conditionals",
          "timestamp": "2020-05-23T00:08:58-05:00",
          "tree_id": "f11491b18f055e9ce4a322ff232588bc105f93c2",
          "url": "https://github.com/christianpoveda/pijama/commit/985c750b7ae5851db496ece9e803e53ac6a8089d"
        },
        "date": 1590210912125,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 27825,
            "range": "± 2077",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 50480,
            "range": "± 5207",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 70131,
            "range": "± 6407",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 128737,
            "range": "± 15390",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 539624,
            "range": "± 42098",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 107431,
            "range": "± 9525",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 842579,
            "range": "± 54471",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 398560,
            "range": "± 40499",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 155014,
            "range": "± 13553",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 166905,
            "range": "± 13920",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1684,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1411,
            "range": "± 172",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7d734e759602ec853fdaeebefd5a4316490977c1",
          "message": "Merge pull request #76 from seanchen1991/master\n\nUpdate tests to use `elif` and run fmt",
          "timestamp": "2020-05-23T00:43:01-05:00",
          "tree_id": "a1a67cd16b5c45b19a8ab92c87df02ad4e3465a1",
          "url": "https://github.com/christianpoveda/pijama/commit/7d734e759602ec853fdaeebefd5a4316490977c1"
        },
        "date": 1590212948441,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26605,
            "range": "± 1229",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 52945,
            "range": "± 2552",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 71047,
            "range": "± 3721",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 121986,
            "range": "± 3242",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 547419,
            "range": "± 19669",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 108007,
            "range": "± 4027",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 843177,
            "range": "± 27687",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 437198,
            "range": "± 16832",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 172030,
            "range": "± 11511",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 180663,
            "range": "± 7983",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1920,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1612,
            "range": "± 66",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ae7545d355c6f521a5acea406be722e200e974f3",
          "message": "Merge pull request #73 from christianpoveda/fix-scoping\n\nFix scoping",
          "timestamp": "2020-05-23T00:54:23-05:00",
          "tree_id": "691d060dbf88950a7b8b11cd960b113dfc23c625",
          "url": "https://github.com/christianpoveda/pijama/commit/ae7545d355c6f521a5acea406be722e200e974f3"
        },
        "date": 1590213610506,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26411,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 53398,
            "range": "± 1997",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 71038,
            "range": "± 2038",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 122113,
            "range": "± 3325",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 548142,
            "range": "± 19773",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 107463,
            "range": "± 2283",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 842118,
            "range": "± 14742",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 436036,
            "range": "± 7355",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 169912,
            "range": "± 6186",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 183237,
            "range": "± 5545",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1939,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1550,
            "range": "± 27",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8e996f871d6d78a3e277cdcdf80ed8e546aa9eb4",
          "message": "Merge pull request #79 from DarkDrek/into-to-from\n\nChange Into to From trait impl",
          "timestamp": "2020-05-23T08:57:39-05:00",
          "tree_id": "f34a5e7aa50ecf8579fa3222c96ac357f48d8e5a",
          "url": "https://github.com/christianpoveda/pijama/commit/8e996f871d6d78a3e277cdcdf80ed8e546aa9eb4"
        },
        "date": 1590242598042,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 24380,
            "range": "± 945",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 49046,
            "range": "± 6614",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 65867,
            "range": "± 2672",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 114152,
            "range": "± 6091",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 508191,
            "range": "± 18839",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 98905,
            "range": "± 4044",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 780830,
            "range": "± 32484",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 402962,
            "range": "± 17202",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 157275,
            "range": "± 9794",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 168214,
            "range": "± 5729",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1856,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1500,
            "range": "± 55",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c47ff9c78dd16982dc7cd3e71aa70ddea3a4731d",
          "message": "Merge pull request #82 from DarkDrek/number-i64\n\nUse i64 instead of i128",
          "timestamp": "2020-05-23T12:31:25-05:00",
          "tree_id": "c096401899730342221e85886352540c4f322f69",
          "url": "https://github.com/christianpoveda/pijama/commit/c47ff9c78dd16982dc7cd3e71aa70ddea3a4731d"
        },
        "date": 1590255421581,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 21496,
            "range": "± 1521",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 43996,
            "range": "± 2918",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 58926,
            "range": "± 4874",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 98531,
            "range": "± 6260",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 441141,
            "range": "± 28525",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 87853,
            "range": "± 6446",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 677859,
            "range": "± 35126",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 349020,
            "range": "± 22403",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 138442,
            "range": "± 10364",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 151474,
            "range": "± 9994",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1540,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1294,
            "range": "± 93",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6bda1e6ffe767dc6a5bb5df9b5702df5c5062908",
          "message": "Merge pull request #86 from DarkDrek/number-bases\n\nParse hex, bin and oct numbers",
          "timestamp": "2020-05-23T19:20:25-05:00",
          "tree_id": "e8b5ec63caf449e048f44d0e4c49aa068b90e0b0",
          "url": "https://github.com/christianpoveda/pijama/commit/6bda1e6ffe767dc6a5bb5df9b5702df5c5062908"
        },
        "date": 1590279990728,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 24224,
            "range": "± 1690",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 50583,
            "range": "± 3201",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 64513,
            "range": "± 5253",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 108642,
            "range": "± 8456",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 488431,
            "range": "± 30900",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 96763,
            "range": "± 8339",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 776955,
            "range": "± 46826",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 416243,
            "range": "± 39382",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 162276,
            "range": "± 12429",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 177675,
            "range": "± 9197",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1720,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1457,
            "range": "± 87",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1deb7893b56d1401ab4c7cfae263d46a6a129035",
          "message": "Merge pull request #87 from DarkDrek/remove-literal-tagging-lir\n\nUse i64 for all literals in lir",
          "timestamp": "2020-05-24T12:41:31-05:00",
          "tree_id": "866f1d0e079208420c767f687840f41599f9e07b",
          "url": "https://github.com/christianpoveda/pijama/commit/1deb7893b56d1401ab4c7cfae263d46a6a129035"
        },
        "date": 1590342439960,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 19957,
            "range": "± 1127",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 43723,
            "range": "± 3253",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 56316,
            "range": "± 4421",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 94515,
            "range": "± 8898",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 423719,
            "range": "± 29304",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 82452,
            "range": "± 5595",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 646724,
            "range": "± 57546",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 338219,
            "range": "± 35555",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 136401,
            "range": "± 8662",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 147506,
            "range": "± 8412",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1439,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1209,
            "range": "± 83",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c31c49f47524ae7ff494407dff01cbbc3f2f159e",
          "message": "Merge pull request #89 from seanchen1991/comments\n\nAdd parsing of single comments",
          "timestamp": "2020-05-25T20:57:12-05:00",
          "tree_id": "089e70604bed86a726abc487adbfb6205c001063",
          "url": "https://github.com/christianpoveda/pijama/commit/c31c49f47524ae7ff494407dff01cbbc3f2f159e"
        },
        "date": 1590458581106,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26400,
            "range": "± 638",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 54918,
            "range": "± 1278",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 72480,
            "range": "± 1549",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 127266,
            "range": "± 3754",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 564195,
            "range": "± 23185",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 110060,
            "range": "± 11546",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 846058,
            "range": "± 10242",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 454839,
            "range": "± 9248",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 184051,
            "range": "± 3377",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 195194,
            "range": "± 4159",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1958,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1534,
            "range": "± 19",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3b7866f066bf7cdaff8dfe0c0d087e50d0b16f67",
          "message": "Merge pull request #90 from seanchen1991/readme-fixes\n\nCleaning up some docs",
          "timestamp": "2020-05-26T13:59:05-05:00",
          "tree_id": "172f914b974f40e73637b2a7245e3e5784ff597f",
          "url": "https://github.com/christianpoveda/pijama/commit/3b7866f066bf7cdaff8dfe0c0d087e50d0b16f67"
        },
        "date": 1590519902371,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 23069,
            "range": "± 1325",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 47076,
            "range": "± 2997",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 61998,
            "range": "± 3893",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 109487,
            "range": "± 7081",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 483716,
            "range": "± 29852",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 90650,
            "range": "± 8091",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 707279,
            "range": "± 45023",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 370006,
            "range": "± 39617",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 153950,
            "range": "± 11204",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 168800,
            "range": "± 8372",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1673,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1253,
            "range": "± 80",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5c563780689d6c350a13e597cfe3f1a131c88f01",
          "message": "Merge pull request #94 from christianpoveda/cli-args\n\nUse structopt to fetch command-line options",
          "timestamp": "2020-05-28T17:14:33-05:00",
          "tree_id": "e164768020be483987877c45e65b352b7c63b9fa",
          "url": "https://github.com/christianpoveda/pijama/commit/5c563780689d6c350a13e597cfe3f1a131c88f01"
        },
        "date": 1590704482546,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26034,
            "range": "± 1478",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 55191,
            "range": "± 2886",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 71025,
            "range": "± 5860",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 118242,
            "range": "± 5479",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 539487,
            "range": "± 24622",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 106357,
            "range": "± 4940",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 809305,
            "range": "± 40032",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 438093,
            "range": "± 19373",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 173814,
            "range": "± 8921",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 186863,
            "range": "± 8750",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1866,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1525,
            "range": "± 76",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "10ce2cfecb473bb1f139a5667f6fd4fb38041a80",
          "message": "Merge pull request #95 from seanchen1991/crates\n\nMoving ast module to pijama_ast crate",
          "timestamp": "2020-05-29T13:44:17-05:00",
          "tree_id": "3060f137d5f59adc8d21a21beb7b335134e84446",
          "url": "https://github.com/christianpoveda/pijama/commit/10ce2cfecb473bb1f139a5667f6fd4fb38041a80"
        },
        "date": 1590778252660,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26402,
            "range": "± 524",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 54867,
            "range": "± 1544",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 72424,
            "range": "± 1212",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 125982,
            "range": "± 4820",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 563209,
            "range": "± 15820",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 110495,
            "range": "± 2685",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 847004,
            "range": "± 49572",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 452569,
            "range": "± 22671",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 184685,
            "range": "± 6457",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 197440,
            "range": "± 6075",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1924,
            "range": "± 1746",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1539,
            "range": "± 64",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "668d187d4096ce6c956ed60fd89076df31f0f490",
          "message": "Merge pull request #100 from christianpoveda/split-ty\n\nAdd new `Ty` for type-checking",
          "timestamp": "2020-05-29T15:59:01-05:00",
          "tree_id": "7094aea14ba28bfa56fdab6143ac027c58980cb3",
          "url": "https://github.com/christianpoveda/pijama/commit/668d187d4096ce6c956ed60fd89076df31f0f490"
        },
        "date": 1590786322541,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 25460,
            "range": "± 803",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 52474,
            "range": "± 1565",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 69847,
            "range": "± 2969",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 121925,
            "range": "± 5400",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 542557,
            "range": "± 23263",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 106741,
            "range": "± 2260",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 811629,
            "range": "± 22904",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 437579,
            "range": "± 8012",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 176453,
            "range": "± 4872",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 190040,
            "range": "± 11062",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1882,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1534,
            "range": "± 51",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cb320feaa62bc5aa56488871bfddbe10db8727cc",
          "message": "Merge pull request #91 from christianpoveda/constraint-typing\n\nDo constraint-based typing instead of check-based typing",
          "timestamp": "2020-05-29T16:16:48-05:00",
          "tree_id": "a35b3a2bc87cf152df2fabd1fff091d11737f3ad",
          "url": "https://github.com/christianpoveda/pijama/commit/cb320feaa62bc5aa56488871bfddbe10db8727cc"
        },
        "date": 1590787401243,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 21664,
            "range": "± 1510",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 51452,
            "range": "± 2787",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 59160,
            "range": "± 4179",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 105106,
            "range": "± 7476",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 456687,
            "range": "± 30683",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 92739,
            "range": "± 8657",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 798203,
            "range": "± 39582",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 380954,
            "range": "± 25354",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 158209,
            "range": "± 8515",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 170842,
            "range": "± 14858",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1696,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1298,
            "range": "± 90",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef378a02b6f98787370d055dd8283a2b94375f65",
          "message": "Merge pull request #99 from christianpoveda/once-cell\n\nChange lazy_static for once_cell",
          "timestamp": "2020-05-29T16:17:14-05:00",
          "tree_id": "ad0695be9567359d89300052d74c029595414342",
          "url": "https://github.com/christianpoveda/pijama/commit/ef378a02b6f98787370d055dd8283a2b94375f65"
        },
        "date": 1590787405612,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 25288,
            "range": "± 561",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 53669,
            "range": "± 1815",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 70609,
            "range": "± 2642",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 123812,
            "range": "± 3318",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 538537,
            "range": "± 20174",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 107219,
            "range": "± 1816",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 818833,
            "range": "± 19786",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 433836,
            "range": "± 12533",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 172636,
            "range": "± 5404",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 184384,
            "range": "± 4177",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1863,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1499,
            "range": "± 31",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "55b1d2c9d211c782f5c50253d00a3fdf7b833981",
          "message": "Merge pull request #102 from christianpoveda/ty-docs\n\nUpdate `pijama::ty` docs",
          "timestamp": "2020-05-29T17:46:47-05:00",
          "tree_id": "c595546f56ee065c452ec38da5390a24334d5dc0",
          "url": "https://github.com/christianpoveda/pijama/commit/55b1d2c9d211c782f5c50253d00a3fdf7b833981"
        },
        "date": 1590792787617,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 26469,
            "range": "± 870",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 54772,
            "range": "± 2094",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 71536,
            "range": "± 5149",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 127131,
            "range": "± 1795",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 558279,
            "range": "± 9509",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 112428,
            "range": "± 4031",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 872870,
            "range": "± 20157",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 452233,
            "range": "± 8122",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 178029,
            "range": "± 4913",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 191365,
            "range": "± 4156",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1883,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1525,
            "range": "± 34",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eae0b4587d3bb0f6ab59376b0fadbe83ddb3af04",
          "message": "Merge pull request #103 from christianpoveda/revamp-location\n\nRevamp `Location` and `Located`",
          "timestamp": "2020-05-29T20:31:40-05:00",
          "tree_id": "3cfeb15b151d5a334a572ab4733a97f5eee181fd",
          "url": "https://github.com/christianpoveda/pijama/commit/eae0b4587d3bb0f6ab59376b0fadbe83ddb3af04"
        },
        "date": 1590802674259,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 24367,
            "range": "± 589",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 50739,
            "range": "± 1175",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 67488,
            "range": "± 1799",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 114928,
            "range": "± 9697",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 517917,
            "range": "± 16612",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 103080,
            "range": "± 2304",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 776219,
            "range": "± 17664",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 415968,
            "range": "± 11989",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 163448,
            "range": "± 5902",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 176265,
            "range": "± 6661",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1776,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1443,
            "range": "± 42",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ec9723c6fc52d4164f08717aa365676a11f1667d",
          "message": "Merge pull request #104 from DarkDrek/move-span\n\nMove Span use to correct crate",
          "timestamp": "2020-05-30T09:57:22-05:00",
          "tree_id": "85d4e92a95d9002b0da0efede31cfcace98b38d8",
          "url": "https://github.com/christianpoveda/pijama/commit/ec9723c6fc52d4164f08717aa365676a11f1667d"
        },
        "date": 1590851033399,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 25483,
            "range": "± 1173",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 51653,
            "range": "± 3043",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 69112,
            "range": "± 5293",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 119274,
            "range": "± 8574",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 524911,
            "range": "± 26014",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 103799,
            "range": "± 7974",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 777491,
            "range": "± 35936",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 424672,
            "range": "± 30738",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 167593,
            "range": "± 8515",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 177637,
            "range": "± 7656",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1809,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1470,
            "range": "± 93",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ddf7295f400dd47a9c7b83a96702d1d832775a04",
          "message": "Merge pull request #97 from christianpoveda/check-overflows\n\nAdd checked arithmetic v2",
          "timestamp": "2020-05-30T11:10:34-05:00",
          "tree_id": "d984ac3aeaa2d95298042d5e900e940dd440277d",
          "url": "https://github.com/christianpoveda/pijama/commit/ddf7295f400dd47a9c7b83a96702d1d832775a04"
        },
        "date": 1590855432825,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 25310,
            "range": "± 1202",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 52904,
            "range": "± 1863",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 68966,
            "range": "± 2083",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 128309,
            "range": "± 4831",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 542491,
            "range": "± 40867",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 122350,
            "range": "± 1860",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 1054859,
            "range": "± 144974",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 437199,
            "range": "± 18274",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 172082,
            "range": "± 6295",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 183290,
            "range": "± 5461",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1821,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1478,
            "range": "± 114",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "52073417686f8d73d68208bdc142a2cd331a1fd9",
          "message": "Merge pull request #106 from seanchen1991/ast_docs\n\nAdd doc comments to pijama_ast",
          "timestamp": "2020-05-31T15:55:12-05:00",
          "tree_id": "58aea8590b3da1b478b76e593de2784739864c8d",
          "url": "https://github.com/christianpoveda/pijama/commit/52073417686f8d73d68208bdc142a2cd331a1fd9"
        },
        "date": 1590958862932,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 19953,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 46596,
            "range": "± 2079",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 52559,
            "range": "± 2697",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 92776,
            "range": "± 5580",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 477978,
            "range": "± 38984",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 82528,
            "range": "± 5202",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 696442,
            "range": "± 57847",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 342926,
            "range": "± 19924",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 131555,
            "range": "± 6642",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 153067,
            "range": "± 17259",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1414,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1299,
            "range": "± 67",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "441427ddc77b2109f1f443ab683a571c59df9241",
          "message": "Merge pull request #109 from christianpoveda/revamp-ast\n\nRevamp Pijama's AST module",
          "timestamp": "2020-05-31T16:15:43-05:00",
          "tree_id": "96f345f8f4c839268c056a74e577be85f8ff4b60",
          "url": "https://github.com/christianpoveda/pijama/commit/441427ddc77b2109f1f443ab683a571c59df9241"
        },
        "date": 1590960107088,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 21317,
            "range": "± 2217",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 43020,
            "range": "± 3191",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 56573,
            "range": "± 6272",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 98303,
            "range": "± 6691",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 460604,
            "range": "± 26677",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 94279,
            "range": "± 5948",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 705892,
            "range": "± 42932",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 377114,
            "range": "± 17909",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 140623,
            "range": "± 9834",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 150227,
            "range": "± 11481",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1467,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1403,
            "range": "± 252",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f7f2ea949a391ff9dc1c491e9bd11c535a8e5e4e",
          "message": "Merge pull request #107 from christianpoveda/prim-trigger-eval\n\nTrigger evaluation when working with primitives",
          "timestamp": "2020-05-31T17:52:28-05:00",
          "tree_id": "64de703c8aafc351aad82a121fd6fb99c97b8208",
          "url": "https://github.com/christianpoveda/pijama/commit/f7f2ea949a391ff9dc1c491e9bd11c535a8e5e4e"
        },
        "date": 1590965975596,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6201,
            "range": "± 448",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 8198,
            "range": "± 488",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 39820,
            "range": "± 4542",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 106797,
            "range": "± 5317",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 403231,
            "range": "± 18734",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 99753,
            "range": "± 6477",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 638071,
            "range": "± 41179",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 404389,
            "range": "± 30661",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 170164,
            "range": "± 9772",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 184907,
            "range": "± 8434",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1755,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1391,
            "range": "± 77",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2c79a4f8a9d67bc89dfc23bff47f41011c740e0b",
          "message": "Merge pull request #110 from christianpoveda/workspace\n\nSplit codebase into a cargo workspace",
          "timestamp": "2020-05-31T18:38:22-05:00",
          "tree_id": "3ddd61a628774330d374e8a4e4609fec85c745c6",
          "url": "https://github.com/christianpoveda/pijama/commit/2c79a4f8a9d67bc89dfc23bff47f41011c740e0b"
        },
        "date": 1590968652629,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5453,
            "range": "± 385",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 6996,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 35680,
            "range": "± 2109",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 105928,
            "range": "± 5816",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 363028,
            "range": "± 19913",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 96993,
            "range": "± 4468",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 668977,
            "range": "± 44992",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 402415,
            "range": "± 17643",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 156922,
            "range": "± 10140",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 172539,
            "range": "± 10360",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1721,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1311,
            "range": "± 77",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d2f1be0a01b30b7ccfeb06fb0a0d5a3f463e5d1e",
          "message": "Merge pull request #111 from christianpoveda/remove-hole\n\nremove Term::Hole",
          "timestamp": "2020-06-01T11:16:11-05:00",
          "tree_id": "237ec7fa76edbb1dc5679a2ab4da684d745eeb8d",
          "url": "https://github.com/christianpoveda/pijama/commit/d2f1be0a01b30b7ccfeb06fb0a0d5a3f463e5d1e"
        },
        "date": 1591028534988,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6259,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7803,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 39294,
            "range": "± 1195",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 110529,
            "range": "± 4763",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 408986,
            "range": "± 12646",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 100261,
            "range": "± 7119",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 660188,
            "range": "± 13739",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 405806,
            "range": "± 14148",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 169798,
            "range": "± 3439",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 180220,
            "range": "± 3748",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1721,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1380,
            "range": "± 49",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e1f0f836d010d68da524c3f5aab33ef7be817a67",
          "message": "Merge pull request #112 from christianpoveda/new-ast\n\nNew AST and parser",
          "timestamp": "2020-06-06T13:52:57-05:00",
          "tree_id": "bfba53befc66c08c9676f597dfe5a5190c4f810c",
          "url": "https://github.com/christianpoveda/pijama/commit/e1f0f836d010d68da524c3f5aab33ef7be817a67"
        },
        "date": 1591470088190,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5907,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7740,
            "range": "± 298",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 37448,
            "range": "± 1377",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 103221,
            "range": "± 4447",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 383424,
            "range": "± 14365",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 93419,
            "range": "± 3218",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 692956,
            "range": "± 27566",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 411115,
            "range": "± 13788",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 170330,
            "range": "± 5697",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 179340,
            "range": "± 8180",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1679,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1323,
            "range": "± 65",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5ba28f8ca2b9eb933b9bb072976e11b216b768ee",
          "message": "Merge pull request #113 from christianpoveda/better-parse-errors\n\nBetter parsing errors",
          "timestamp": "2020-06-06T20:26:01-05:00",
          "tree_id": "c3da24b31ae65e14fe7637e9cf890751fcf5e17f",
          "url": "https://github.com/christianpoveda/pijama/commit/5ba28f8ca2b9eb933b9bb072976e11b216b768ee"
        },
        "date": 1591493613585,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5665,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7116,
            "range": "± 472",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 36892,
            "range": "± 1817",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 100433,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 372963,
            "range": "± 19771",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 90185,
            "range": "± 3979",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 590784,
            "range": "± 36145",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 372245,
            "range": "± 16418",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 158837,
            "range": "± 7216",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 168716,
            "range": "± 7371",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1656,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1261,
            "range": "± 58",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "67727c0d650475c4b43c3491902a3972bb731d22",
          "message": "Merge pull request #114 from christianpoveda/fix-scoping\n\nFix scoping and type checking of recursive functions",
          "timestamp": "2020-06-08T18:24:43-05:00",
          "tree_id": "9a57089fdd38f7926c636ea71b172303bfa0f252",
          "url": "https://github.com/christianpoveda/pijama/commit/67727c0d650475c4b43c3491902a3972bb731d22"
        },
        "date": 1591659164453,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6184,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7917,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 40500,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 113157,
            "range": "± 1659",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 407430,
            "range": "± 7962",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 102636,
            "range": "± 2484",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 654750,
            "range": "± 7598",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 411219,
            "range": "± 6947",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 177230,
            "range": "± 1730",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 189303,
            "range": "± 4464",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1770,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1397,
            "range": "± 19",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f00050dfbac6668d342a07d0651a2ae93958587",
          "message": "Merge pull request #115 from christianpoveda/split-core\n\nsplit remaining core modules",
          "timestamp": "2020-06-09T09:26:58-05:00",
          "tree_id": "4bf7bdf01a77e8f609cbbaad8e48df35c59f2d45",
          "url": "https://github.com/christianpoveda/pijama/commit/5f00050dfbac6668d342a07d0651a2ae93958587"
        },
        "date": 1591713346606,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6443,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 8620,
            "range": "± 510",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 40207,
            "range": "± 2782",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 110151,
            "range": "± 12189",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 441115,
            "range": "± 27740",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 108838,
            "range": "± 6979",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 678344,
            "range": "± 43397",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 409048,
            "range": "± 24896",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 177068,
            "range": "± 14069",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 201106,
            "range": "± 12143",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1866,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1391,
            "range": "± 78",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd002ef28f0ff0789296cc842903ae51e5a1a6c7",
          "message": "Merge pull request #116 from christianpoveda/finish-tests\n\nAdd missing tycheck tests",
          "timestamp": "2020-06-09T09:59:06-05:00",
          "tree_id": "cd0dce867bb93e98dd3680046adf28ef7a0d1e9a",
          "url": "https://github.com/christianpoveda/pijama/commit/fd002ef28f0ff0789296cc842903ae51e5a1a6c7"
        },
        "date": 1591715241209,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6219,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7930,
            "range": "± 417",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 40541,
            "range": "± 1764",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 111298,
            "range": "± 7323",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 413176,
            "range": "± 25609",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 108382,
            "range": "± 7751",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 658150,
            "range": "± 51970",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 417291,
            "range": "± 27876",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 182605,
            "range": "± 12948",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 196026,
            "range": "± 7005",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1772,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1406,
            "range": "± 35",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "24089ef564ed1534d8208ad93c27c10c08a870f5",
          "message": "Merge pull request #117 from christianpoveda/finish-benches\n\nAdd missing benchmarks",
          "timestamp": "2020-06-09T10:18:36-05:00",
          "tree_id": "7bbb2e0cd60d7ed9dd655c81424ed5ed4762d159",
          "url": "https://github.com/christianpoveda/pijama/commit/24089ef564ed1534d8208ad93c27c10c08a870f5"
        },
        "date": 1591716412051,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6131,
            "range": "± 740",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7759,
            "range": "± 606",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 38529,
            "range": "± 4090",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 104828,
            "range": "± 5695",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 394750,
            "range": "± 30405",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 93949,
            "range": "± 11115",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 602803,
            "range": "± 40018",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 367491,
            "range": "± 22467",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 160500,
            "range": "± 15977",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 158438,
            "range": "± 8007",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1531,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1273,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "cond_chain",
            "value": 20696,
            "range": "± 2727",
            "unit": "ns/iter"
          },
          {
            "name": "short_circuit",
            "value": 8977,
            "range": "± 547",
            "unit": "ns/iter"
          },
          {
            "name": "adler32",
            "value": 17856,
            "range": "± 942",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8dec0f14423e1d1bc8a4e853b16dce46554cc949",
          "message": "Merge pull request #120 from christianpoveda/common\n\nmove common types to new crate",
          "timestamp": "2020-08-01T17:23:48-05:00",
          "tree_id": "50b7537099cd009c53fd04c28c89118f6be3f770",
          "url": "https://github.com/christianpoveda/pijama/commit/8dec0f14423e1d1bc8a4e853b16dce46554cc949"
        },
        "date": 1596321203045,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5445,
            "range": "± 390",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 7075,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 34276,
            "range": "± 1117",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 96408,
            "range": "± 3864",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 351669,
            "range": "± 19117",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 84522,
            "range": "± 2732",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 553907,
            "range": "± 29208",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 350054,
            "range": "± 14575",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 149075,
            "range": "± 6478",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 161750,
            "range": "± 4153",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1510,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1281,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cond_chain",
            "value": 20320,
            "range": "± 959",
            "unit": "ns/iter"
          },
          {
            "name": "short_circuit",
            "value": 9132,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "adler32",
            "value": 16672,
            "range": "± 734",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "365d16d535c883367b52eaee2a011d35d940d396",
          "message": "Merge pull request #121 from christianpoveda/simplify-mir\n\nSimplify MIR",
          "timestamp": "2020-08-03T22:23:23-05:00",
          "tree_id": "0ab8fc555f62dd8d446ddf01ad1db98e4d11796b",
          "url": "https://github.com/christianpoveda/pijama/commit/365d16d535c883367b52eaee2a011d35d940d396"
        },
        "date": 1596511950164,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5557,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 6996,
            "range": "± 522",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 35244,
            "range": "± 2947",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 96864,
            "range": "± 8594",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 350851,
            "range": "± 35122",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 93173,
            "range": "± 6243",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 570837,
            "range": "± 46581",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 349585,
            "range": "± 23858",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 147429,
            "range": "± 12275",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 159231,
            "range": "± 16512",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1442,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1310,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "cond_chain",
            "value": 18929,
            "range": "± 1843",
            "unit": "ns/iter"
          },
          {
            "name": "short_circuit",
            "value": 8468,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "adler32",
            "value": 16457,
            "range": "± 1454",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ae9d5ef90869befe8743b05c0dd873e3c3c239a9",
          "message": "Merge pull request #122 from christianpoveda/new_mir\n\nNew MIR",
          "timestamp": "2020-08-07T12:25:57-05:00",
          "tree_id": "04c7d42d78bddecbc11b048d6d43e52efb18fd2f",
          "url": "https://github.com/christianpoveda/pijama/commit/ae9d5ef90869befe8743b05c0dd873e3c3c239a9"
        },
        "date": 1596821749053,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 6415,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 8460,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 41827,
            "range": "± 1963",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 111909,
            "range": "± 4826",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 426714,
            "range": "± 17316",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 103433,
            "range": "± 3150",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 669471,
            "range": "± 43558",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 419817,
            "range": "± 10010",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 178860,
            "range": "± 4524",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 199852,
            "range": "± 6143",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1868,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1463,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cond_chain",
            "value": 23154,
            "range": "± 1097",
            "unit": "ns/iter"
          },
          {
            "name": "short_circuit",
            "value": 10123,
            "range": "± 412",
            "unit": "ns/iter"
          },
          {
            "name": "adler32",
            "value": 19909,
            "range": "± 721",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "31802960+christianpoveda@users.noreply.github.com",
            "name": "Christian Poveda",
            "username": "christianpoveda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32ffc2fd943090331ac0004f631e54f3b6b7c749",
          "message": "Merge pull request #123 from christianpoveda/tag_terms\n\nAdd context structure to hold all the term metadata",
          "timestamp": "2020-08-09T18:05:15-05:00",
          "tree_id": "525ca2bd113e89d87f8903fe7aec9cf4f2ae4d29",
          "url": "https://github.com/christianpoveda/pijama/commit/32ffc2fd943090331ac0004f631e54f3b6b7c749"
        },
        "date": 1597014831579,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic",
            "value": 5109,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "logic",
            "value": 6643,
            "range": "± 494",
            "unit": "ns/iter"
          },
          {
            "name": "factorial",
            "value": 32099,
            "range": "± 2927",
            "unit": "ns/iter"
          },
          {
            "name": "factorial_tail",
            "value": 86653,
            "range": "± 8345",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci",
            "value": 319443,
            "range": "± 26019",
            "unit": "ns/iter"
          },
          {
            "name": "fibonacci_tail",
            "value": 77494,
            "range": "± 5786",
            "unit": "ns/iter"
          },
          {
            "name": "gcd",
            "value": 526517,
            "range": "± 41118",
            "unit": "ns/iter"
          },
          {
            "name": "ackermann",
            "value": 322409,
            "range": "± 22453",
            "unit": "ns/iter"
          },
          {
            "name": "calling",
            "value": 138906,
            "range": "± 10187",
            "unit": "ns/iter"
          },
          {
            "name": "complex_calling",
            "value": 154097,
            "range": "± 12399",
            "unit": "ns/iter"
          },
          {
            "name": "fancy_max",
            "value": 1392,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "step",
            "value": 1141,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "cond_chain",
            "value": 17114,
            "range": "± 1198",
            "unit": "ns/iter"
          },
          {
            "name": "short_circuit",
            "value": 7693,
            "range": "± 532",
            "unit": "ns/iter"
          },
          {
            "name": "adler32",
            "value": 15661,
            "range": "± 1253",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}