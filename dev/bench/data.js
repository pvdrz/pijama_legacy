window.BENCHMARK_DATA = {
  "lastUpdate": 1590519902950,
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
      }
    ]
  }
}