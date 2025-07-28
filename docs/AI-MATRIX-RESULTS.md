# AI Matrix Test Results

_Last updated: 28/07/2025, 20:02:13_

## Matrix Table

**Test Configuration:**
  Total games played: 2250
  Duration: 29.34 seconds
  Games per second: 76.7

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 | ML-Default | ML-SelfPlay |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 32.0 | 16.0 | 2.0 | 10.0 | 6.0 | 4.0 | 2.0 | 46.0 | 34.0 |
| Heuristic | 68.0 | - | 54.0 | 48.0 | 50.0 | 46.0 | 0.0 | 0.0 | 100.0 | 100.0 |
| EMM-Depth1 | 84.0 | 46.0 | - | 50.0 | 100.0 | 100.0 | 54.0 | 100.0 | 100.0 | 100.0 |
| EMM-Depth2 | 98.0 | 52.0 | 50.0 | - | 44.0 | 100.0 | 100.0 | 44.0 | 48.0 | 100.0 |
| EMM-Depth3 | 90.0 | 50.0 | 0.0 | 56.0 | - | 50.0 | 0.0 | 48.0 | 100.0 | 100.0 |
| EMM-Depth4 | 94.0 | 54.0 | 0.0 | 0.0 | 50.0 | - | 52.0 | 44.0 | 100.0 | 100.0 |
| EMM-Depth5 | 96.0 | 100.0 | 46.0 | 0.0 | 100.0 | 48.0 | - | 0.0 | 100.0 | 58.0 |
| EMM-Depth6 | 98.0 | 100.0 | 0.0 | 56.0 | 52.0 | 56.0 | 100.0 | - | 100.0 | 54.0 |
| ML-Default | 54.0 | 0.0 | 0.0 | 52.0 | 0.0 | 0.0 | 0.0 | 0.0 | - | 50.0 |
| ML-SelfPlay | 66.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 42.0 | 46.0 | 50.0 | - |


## Performance Summary

1. EMM-Depth1: 81.6% average win rate
2. EMM-Depth2: 70.7% average win rate
3. EMM-Depth6: 68.4% average win rate
4. EMM-Depth5: 60.9% average win rate
5. EMM-Depth3: 54.9% average win rate
6. EMM-Depth4: 54.9% average win rate
7. Heuristic: 51.8% average win rate
8. ML-SelfPlay: 22.7% average win rate
9. ML-Default: 17.3% average win rate
10. Random: 16.9% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Heuristic | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| Random | 0.0 | Very Fast |
| EMM-Depth2 | 0.0 | Very Fast |
| ML-SelfPlay | 0.0 | Very Fast |
| ML-Default | 0.0 | Very Fast |
| EMM-Depth3 | 4.6 | Fast |
| EMM-Depth4 | 25.9 | Moderate |
| EMM-Depth5 | 84.3 | Slow |
| EMM-Depth6 | 262.1 | Slow |


## Recommendations

- EMM-Depth1 shows excellent performance (81.6% avg win rate) and is ready for production
- Heuristic is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
