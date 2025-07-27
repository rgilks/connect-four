# AI Matrix Test Results

_Last updated: 27/07/2025, 02:22:25_

## Matrix Table

**Test Configuration:**
  Total games played: 1400
  Duration: 15.73 seconds
  Games per second: 89.0

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 44.0 | 48.0 | 0.0 | 2.0 | 2.0 | 0.0 | 2.0 |
| Heuristic | 56.0 | - | 48.0 | 20.0 | 24.0 | 0.0 | 92.0 | 0.0 |
| EMM-Depth1 | 52.0 | 52.0 | - | 16.0 | 44.0 | 12.0 | 46.0 | 0.0 |
| EMM-Depth2 | 100.0 | 80.0 | 84.0 | - | 52.0 | 16.0 | 54.0 | 48.0 |
| EMM-Depth3 | 98.0 | 76.0 | 56.0 | 48.0 | - | 40.0 | 0.0 | 2.0 |
| EMM-Depth4 | 98.0 | 100.0 | 88.0 | 84.0 | 60.0 | - | 50.0 | 54.0 |
| EMM-Depth5 | 100.0 | 8.0 | 54.0 | 46.0 | 100.0 | 50.0 | - | 16.0 |
| EMM-Depth6 | 98.0 | 100.0 | 100.0 | 52.0 | 98.0 | 46.0 | 84.0 | - |


## Performance Summary

1. EMM-Depth6: 82.6% average win rate
2. EMM-Depth4: 76.3% average win rate
3. EMM-Depth2: 62.0% average win rate
4. EMM-Depth5: 53.4% average win rate
5. EMM-Depth3: 45.7% average win rate
6. Heuristic: 34.3% average win rate
7. EMM-Depth1: 31.7% average win rate
8. Random: 14.0% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| EMM-Depth2 | 0.0 | Very Fast |
| EMM-Depth3 | 1.2 | Fast |
| EMM-Depth4 | 9.1 | Fast |
| EMM-Depth5 | 30.8 | Moderate |
| EMM-Depth6 | 95.3 | Slow |


## Recommendations

- EMM-Depth6 shows excellent performance (82.6% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
