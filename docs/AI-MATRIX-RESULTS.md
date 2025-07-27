# AI Matrix Test Results

_Last updated: 27/07/2025, 01:22:42_

## Matrix Table

**Test Configuration:**
  Total games played: 720
  Duration: 3.89 seconds
  Games per second: 184.9

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 | EMM-Depth7 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 45.0 | 70.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Heuristic | 55.0 | - | 50.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| EMM-Depth1 | 30.0 | 50.0 | - | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| EMM-Depth2 | 100.0 | 100.0 | 100.0 | - | 50.0 | 50.0 | 0.0 | 50.0 | 0.0 |
| EMM-Depth3 | 100.0 | 100.0 | 100.0 | 50.0 | - | 50.0 | 0.0 | 50.0 | 50.0 |
| EMM-Depth4 | 100.0 | 100.0 | 100.0 | 50.0 | 50.0 | - | 50.0 | 50.0 | 100.0 |
| EMM-Depth5 | 100.0 | 100.0 | 100.0 | 100.0 | 100.0 | 50.0 | - | 100.0 | 100.0 |
| EMM-Depth6 | 100.0 | 100.0 | 100.0 | 50.0 | 50.0 | 50.0 | 0.0 | - | 0.0 |
| EMM-Depth7 | 100.0 | 100.0 | 100.0 | 100.0 | 50.0 | 0.0 | 0.0 | 100.0 | - |


## Performance Summary

1. EMM-Depth5: 93.8% average win rate
2. EMM-Depth4: 75.0% average win rate
3. EMM-Depth7: 68.8% average win rate
4. EMM-Depth3: 62.5% average win rate
5. EMM-Depth6: 56.2% average win rate
6. EMM-Depth2: 56.2% average win rate
7. Random: 14.4% average win rate
8. Heuristic: 13.1% average win rate
9. EMM-Depth1: 10.0% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| EMM-Depth2 | 0.0 | Very Fast |
| Random | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| EMM-Depth3 | 0.0 | Very Fast |
| EMM-Depth4 | 0.4 | Very Fast |
| EMM-Depth5 | 4.4 | Fast |
| EMM-Depth6 | 13.5 | Moderate |
| EMM-Depth7 | 41.9 | Moderate |


## Recommendations

- EMM-Depth5 shows excellent performance (93.8% avg win rate) and is ready for production
- EMM-Depth2 is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
