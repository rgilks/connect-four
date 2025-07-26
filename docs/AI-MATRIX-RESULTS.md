# AI Matrix Test Results

_Last updated: 26/07/2025, 23:52:45_

## Matrix Table

**Test Configuration:**
  Total games played: 500
  Duration: 1.22 seconds
  Games per second: 409.6

| AI Type | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 |
| --- | --- | --- | --- | --- | --- |
| Random | - | 100.0 | 80.0 | 82.0 | 56.0 |
| Heuristic | 0.0 | - | 50.0 | 0.0 | 50.0 |
| EMM-Depth1 | 20.0 | 50.0 | - | 50.0 | 0.0 |
| EMM-Depth2 | 18.0 | 100.0 | 50.0 | - | 0.0 |
| EMM-Depth3 | 44.0 | 50.0 | 100.0 | 100.0 | - |


## Performance Summary

1. Random: 79.5% average win rate
2. EMM-Depth3: 73.5% average win rate
3. EMM-Depth2: 42.0% average win rate
4. EMM-Depth1: 30.0% average win rate
5. Heuristic: 25.0% average win rate

## Speed Analysis

| AI | ms/move | Speed |
|---|---|---|
| Random | 0.0 | Very Fast |
| EMM-Depth1 | 0.0 | Very Fast |
| Heuristic | 0.0 | Very Fast |
| EMM-Depth2 | 1.0 | Very Fast |
| EMM-Depth3 | 6.9 | Fast |


## Recommendations

- Random shows excellent performance (79.5% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
