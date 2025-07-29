# AI Matrix Test Results

_Last updated: 29/07/2025, 05:26:12_

## Matrix Table

**Test Configuration:**
  Total games played: 1800
  Duration: 27.23 seconds
  Games per second: 66.1

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 | ML-Simple |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Random | - | 34.0 | 20.0 | 28.0 | 22.0 | 26.0 | 36.0 | 8.0 | 32.0 |
| Heuristic | 66.0 | - | 66.0 | 32.0 | 48.0 | 100.0 | 26.0 | 30.0 | 72.0 |
| EMM-Depth1 | 80.0 | 34.0 | - | 56.0 | 50.0 | 100.0 | 50.0 | 50.0 | 50.0 |
| EMM-Depth2 | 72.0 | 68.0 | 44.0 | - | 40.0 | 50.0 | 46.0 | 50.0 | 50.0 |
| EMM-Depth3 | 78.0 | 52.0 | 50.0 | 60.0 | - | 46.0 | 50.0 | 36.0 | 50.0 |
| EMM-Depth4 | 74.0 | 0.0 | 0.0 | 50.0 | 54.0 | - | 52.0 | 18.0 | 50.0 |
| EMM-Depth5 | 64.0 | 74.0 | 50.0 | 54.0 | 50.0 | 48.0 | - | 74.0 | 76.0 |
| EMM-Depth6 | 92.0 | 70.0 | 50.0 | 50.0 | 64.0 | 82.0 | 26.0 | - | 100.0 |
| ML-Simple | 68.0 | 28.0 | 50.0 | 50.0 | 50.0 | 50.0 | 24.0 | 0.0 | - |


## Performance Summary

1. EMM-Depth6: 66.8% average win rate
2. EMM-Depth5: 61.2% average win rate
3. EMM-Depth1: 58.8% average win rate
4. Heuristic: 55.0% average win rate
5. EMM-Depth3: 52.8% average win rate
6. EMM-Depth2: 52.5% average win rate
7. ML-Simple: 40.0% average win rate
8. EMM-Depth4: 37.2% average win rate
9. Random: 25.8% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth2 | 0.3 | Very Fast |
| ML-Simple | 0.5 | Very Fast |
| EMM-Depth3 | 3.7 | Fast |
| EMM-Depth4 | 22.6 | Moderate |
| EMM-Depth5 | 69.7 | Slow |
| EMM-Depth6 | 210.7 | Slow |


## Recommendations

- EMM-Depth6 shows good performance (66.8% avg win rate) and could be used in production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
