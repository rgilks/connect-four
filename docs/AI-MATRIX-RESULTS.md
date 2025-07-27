# AI Matrix Test Results

_Last updated: 27/07/2025, 09:31:14_

> **Note:** These results reflect the latest evolved genetic parameters (July 2025). The AI now achieves significantly higher win rates and faster performance due to full genetic optimization of all evaluation parameters.

## Matrix Table

**Test Configuration:**
Total games played: 1400
Duration: 9.98 seconds
Games per second: 140.2

| AI Type    | Random | Heuristic | EMM-Depth1 | EMM-Depth2 | EMM-Depth3 | EMM-Depth4 | EMM-Depth5 | EMM-Depth6 |
| ---------- | ------ | --------- | ---------- | ---------- | ---------- | ---------- | ---------- | ---------- |
| Random     | -      | 24.0      | 26.0       | 0.0        | 0.0        | 2.0        | 0.0        | 0.0        |
| Heuristic  | 76.0   | -         | 26.0       | 22.0       | 60.0       | 0.0        | 0.0        | 24.0       |
| EMM-Depth1 | 74.0   | 74.0      | -          | 34.0       | 30.0       | 0.0        | 10.0       | 28.0       |
| EMM-Depth2 | 100.0  | 78.0      | 66.0       | -          | 46.0       | 0.0        | 0.0        | 14.0       |
| EMM-Depth3 | 100.0  | 40.0      | 70.0       | 54.0       | -          | 32.0       | 40.0       | 0.0        |
| EMM-Depth4 | 98.0   | 100.0     | 100.0      | 100.0      | 68.0       | -          | 16.0       | 38.0       |
| EMM-Depth5 | 100.0  | 100.0     | 90.0       | 100.0      | 60.0       | 84.0       | -          | 48.0       |
| EMM-Depth6 | 100.0  | 76.0      | 72.0       | 86.0       | 100.0      | 62.0       | 52.0       | -          |

## Performance Summary

1. EMM-Depth5: 83.1% average win rate
2. EMM-Depth6: 78.3% average win rate
3. EMM-Depth4: 74.3% average win rate
4. EMM-Depth3: 48.0% average win rate
5. EMM-Depth2: 43.4% average win rate
6. EMM-Depth1: 35.7% average win rate
7. Heuristic: 29.7% average win rate
8. Random: 7.4% average win rate

## Speed Analysis

| AI         | ms/move | Speed     |
| ---------- | ------- | --------- |
| Random     | 0.0     | Very Fast |
| Heuristic  | 0.0     | Very Fast |
| EMM-Depth2 | 0.0     | Very Fast |
| EMM-Depth1 | 0.0     | Very Fast |
| EMM-Depth3 | 1.3     | Fast      |
| EMM-Depth4 | 8.5     | Fast      |
| EMM-Depth5 | 32.0    | Moderate  |
| EMM-Depth6 | 88.8    | Slow      |

## Recommendations

- EMM-Depth5 shows excellent performance (83.1% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use EMM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes
