window.BENCHMARK_DATA = {
  "lastUpdate": 1590110071606,
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
      }
    ]
  }
}