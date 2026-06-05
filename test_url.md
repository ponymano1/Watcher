### health
curl http://localhost:3000/health  

### 
curl -X POST http://localhost:3000/watch-addresses \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x1111111111111111111111111111111111111111",
    "chain_id": 1,
    "label": "my wallet"
  }'


