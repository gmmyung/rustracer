# Performance log

512x512, 1024 sample per pixel

Before index optimization
- 8 cores: 14.034s
- 4 cores: 17.251s
- 2 cores: 33.171s
- 1 cores: 64.920s


After index optimization
- 8 cores: 13.191s 
  not much improvement M1 macbook has 4 performance cores and 4 efficiency cores. Probably causes more hassle due to mutex locks.
- 4 cores: 13.130s
- 2 cores: 25.237s
- 1 cores: 48.923s

Without progress ba