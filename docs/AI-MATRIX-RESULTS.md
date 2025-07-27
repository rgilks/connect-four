# AI Matrix Test Results

_Last updated: 27/07/2025, 17:02:45_

## Matrix Table

**Test Configuration:**
  Total games played: 1400
  Duration: 10.23 seconds
  Games per second: 136.8

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random     | -    | 36.0 | 6.0 | 2.0 | 0.0 | 2.0 | 4.0 | 4.0 |
| Heuristic  | 64.0 | - | 52.0 | 44.0 | 70.0 | 44.0 | 40.0 | 0.0 |
| EMM-Depth1 | 94.0 | 48.0 | - | 92.0 | 100.0 | 100.0 | 54.0 | 80.0 |
| EMM-Depth2 | 98.0 | 56.0 | 8.0 | - | 46.0 | 50.0 | 100.0 | 26.0 |
| EMM-Depth3 | 100 | 30.0 | 0.0 | 54.0 | - | 10.0 | 0.0 | 16.0 |
| EMM-Depth4 | 98.0 | 56.0 | 0.0 | 50.0 | 90.0 | - | 24.0 | 56.0 |
| EMM-Depth5 | 96.0 | 60.0 | 46.0 | 0.0 | 100.0 | 76.0 | - | 34.0 |
| EMM-Depth6 | 96.0 | 100.0 | 20.0 | 74.0 | 84.0 | 44.0 | 66.0 | - |


## Performance Summary

1. EMM-Depth1: 81.1% average win rate
2. EMM-Depth6: 69.1% average win rate
3. EMM-Depth5: 58.9% average win rate
4. EMM-Depth2: 54.9% average win rate
5. EMM-Depth4: 53.4% average win rate
6. Heuristic: 44.9% average win rate
7. EMM-Depth3: 30.0% average win rate
8. Random: 7.7% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| EMM-Depth2 | 0.0 | Very Fast |
| EMM-Depth3 | 1.1 | Fast |
| EMM-Depth4 | 7.2 | Fast |
| EMM-Depth5 | 25.0 | Moderate |
| EMM-Depth6 | 74.0 | Slow |


## Recommendations

- EMM-Depth1 shows excellent performance (81.1% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
