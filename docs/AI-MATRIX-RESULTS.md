# AI Matrix Test Results

_Last updated: 27/07/2025, 16:32:55_

## Matrix Table

**Test Configuration:**
  Total games played: 1400
  Duration: 10.06 seconds
  Games per second: 139.2

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 32.0 | 2.0 | 2.0 | 6.0 | 4.0 | 6.0 | 2.0 |
| Heuristic | 68.0 | - | 46.0 | 50.0 | 46.0 | 56.0 | 38.0 | 0.0 |
| EMM-Depth1 | 98.0 | 54.0 | - | 52.0 | 100.0 | 100.0 | 66.0 | 80.0 |
| EMM-Depth2 | 98.0 | 50.0 | 48.0 | - | 32.0 | 42.0 | 100.0 | 82.0 |
| EMM-Depth3 | 94.0 | 54.0 | 0.0 | 68.0 | - | 38.0 | 0.0 | 0.0 |
| EMM-Depth4 | 96.0 | 44.0 | 0.0 | 58.0 | 62.0 | - | 26.0 | 50.0 |
| EMM-Depth5 | 94.0 | 62.0 | 34.0 | 0.0 | 100.0 | 74.0 | - | 28.0 |
| EMM-Depth6 | 98.0 | 100.0 | 20.0 | 18.0 | 100.0 | 50.0 | 72.0 | - |


## Performance Summary

1. EMM-Depth1: 78.6% average win rate
2. EMM-Depth6: 65.4% average win rate
3. EMM-Depth2: 64.6% average win rate
4. EMM-Depth5: 56.0% average win rate
5. EMM-Depth4: 48.0% average win rate
6. Heuristic: 43.4% average win rate
7. EMM-Depth3: 36.3% average win rate
8. Random: 7.7% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth2 | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| EMM-Depth3 | 1.2 | Fast |
| EMM-Depth4 | 6.7 | Fast |
| EMM-Depth5 | 25.0 | Moderate |
| EMM-Depth6 | 74.6 | Slow |


## Recommendations

- EMM-Depth1 shows excellent performance (78.6% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
