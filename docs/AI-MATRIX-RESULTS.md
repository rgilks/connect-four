# AI Matrix Test Results

_Last updated: 27/07/2025, 21:56:22_

## Matrix Table

**Test Configuration:**
Total games played: 1400
Duration: 33.52 seconds
Games per second: 41.8

| AI Type    | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 |
| ---------- | ------ | --------- | ---------- | ---------- | ---------- | ---------- | ---------- | ---------- |
| Random     | -      | 28.0      | 4.0        | 6.0        | 2.0        | 4.0        | 2.0        | 0.0        |
| Heuristic  | 72.0   | -         | 46.0       | 52.0       | 60.0       | 50.0       | 0.0        | 0.0        |
| EMM-Depth1 | 96.0   | 54.0      | -          | 40.0       | 100.0      | 100.0      | 42.0       | 100.0      |
| EMM-Depth2 | 94.0   | 48.0      | 60.0       | -          | 60.0       | 100.0      | 100.0      | 44.0       |
| EMM-Depth3 | 98.0   | 40.0      | 0.0        | 40.0       | -          | 50.0       | 0.0        | 50.0       |
| EMM-Depth4 | 96.0   | 50.0      | 0.0        | 0.0        | 50.0       | -          | 56.0       | 48.0       |
| EMM-Depth5 | 98.0   | 100.0     | 58.0       | 0.0        | 100.0      | 44.0       | -          | 0.0        |
| EMM-Depth6 | 100.0  | 100.0     | 0.0        | 56.0       | 50.0       | 52.0       | 100.0      | -          |

## Performance Summary

1. EMM-Depth1: 76.0% average win rate
2. EMM-Depth2: 72.3% average win rate
3. EMM-Depth6: 65.4% average win rate
4. EMM-Depth5: 57.1% average win rate
5. EMM-Depth4: 42.9% average win rate
6. Heuristic: 40.0% average win rate
7. EMM-Depth3: 39.7% average win rate
8. Random: 6.6% average win rate

## Speed Analysis

| AI         | ms/move | Speed     |
| ---------- | ------- | --------- |
| EMM-Depth1 | 0.0     | Very Fast |
| Heuristic  | 0.0     | Very Fast |
| Random     | 0.0     | Very Fast |
| EMM-Depth2 | 0.0     | Very Fast |
| EMM-Depth3 | 4.8     | Fast      |
| EMM-Depth4 | 26.8    | Moderate  |
| EMM-Depth5 | 95.2    | Slow      |
| EMM-Depth6 | 274.1   | Slow      |

## Recommendations

- **EMM-Depth1 is now the production setting** (76.0% avg win rate, 0.0ms/move)
- EMM-Depth1 provides excellent performance with instant response time
- Use EMM-Depth5 for maximum strength when speed is not critical
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
