# AI Matrix Test Results

_Last updated: 28/07/2025, 22:19:19_

## Matrix Table

**Test Configuration:**
  Total games played: 3300
  Duration: 36.54 seconds
  Games per second: 90.3

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 | ML-Default | ML-SelfPlay | ML-Intensive | ML-PolicyFix |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 28.0 | 6.0 | 4.0 | 2.0 | 0.0 | 2.0 | 4.0 | 4.0 | 52.0 | 12.0 | 64.0 |
| Heuristic | 72.0 | - | 48.0 | 70.0 | 34.0 | 34.0 | 0.0 | 0.0 | 100.0 | 74.0 | 80.0 | 100.0 |
| EMM-Depth1 | 94.0 | 52.0 | - | 54.0 | 100.0 | 100.0 | 56.0 | 100.0 | 100.0 | 66.0 | 100.0 | 76.0 |
| EMM-Depth2 | 96.0 | 30.0 | 46.0 | - | 48.0 | 100.0 | 100.0 | 42.0 | 56.0 | 100.0 | 100.0 | 100.0 |
| EMM-Depth3 | 98.0 | 66.0 | 0.0 | 52.0 | - | 50.0 | 0.0 | 50.0 | 50.0 | 100.0 | 100.0 | 78.0 |
| EMM-Depth4 | 100.0 | 66.0 | 0.0 | 0.0 | 50.0 | - | 68.0 | 42.0 | 76.0 | 16.0 | 46.0 | 70.0 |
| EMM-Depth5 | 98.0 | 100.0 | 44.0 | 0.0 | 100.0 | 32.0 | - | 0.0 | 100.0 | 66.0 | 70.0 | 74.0 |
| EMM-Depth6 | 96.0 | 100.0 | 0.0 | 58.0 | 50.0 | 58.0 | 100.0 | - | 100.0 | 72.0 | 70.0 | 58.0 |
| ML-Default | 96.0 | 0.0 | 0.0 | 44.0 | 50.0 | 24.0 | 0.0 | 0.0 | - | 36.0 | 28.0 | 50.0 |
| ML-SelfPlay | 48.0 | 26.0 | 34.0 | 0.0 | 0.0 | 84.0 | 34.0 | 28.0 | 64.0 | - | 50.0 | 34.0 |
| ML-Intensive | 88.0 | 20.0 | 0.0 | 0.0 | 0.0 | 54.0 | 30.0 | 30.0 | 72.0 | 50.0 | - | 26.0 |
| ML-PolicyFix | 36.0 | 0.0 | 24.0 | 0.0 | 22.0 | 30.0 | 26.0 | 42.0 | 50.0 | 66.0 | 74.0 | - |


## Performance Summary

1. EMM-Depth1: 81.6% average win rate
2. EMM-Depth2: 74.4% average win rate
3. EMM-Depth6: 69.3% average win rate
4. EMM-Depth5: 62.2% average win rate
5. EMM-Depth3: 58.5% average win rate
6. Heuristic: 55.6% average win rate
7. EMM-Depth4: 48.5% average win rate
8. ML-SelfPlay: 36.5% average win rate
9. ML-Intensive: 33.6% average win rate
10. ML-PolicyFix: 33.6% average win rate
11. ML-Default: 29.8% average win rate
12. Random: 16.2% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| EMM-Depth2 | 0.1 | Very Fast |
| ML-Default | 0.1 | Very Fast |
| ML-SelfPlay | 0.1 | Very Fast |
| ML-PolicyFix | 0.1 | Very Fast |
| ML-Intensive | 0.1 | Very Fast |
| EMM-Depth3 | 4.6 | Fast |
| EMM-Depth4 | 26.9 | Moderate |
| EMM-Depth5 | 94.8 | Slow |
| EMM-Depth6 | 286.4 | Slow |


## Recommendations

- EMM-Depth1 shows excellent performance (81.6% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
