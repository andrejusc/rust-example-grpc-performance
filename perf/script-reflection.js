// To be executed via k6 in a way like this:
// k6 run perf/script-reflection.js
// Or when trying concurrent load:
// k6 run --vus 5 --iterations 100 perf/script-reflection.js

import grpc from 'k6/net/grpc';
import { check, sleep } from 'k6';

const client = new grpc.Client();

export default () => {
  client.connect('localhost:9090', {
    plaintext: true,
    reflect: true
  });

  const data = { vote: 0, url: 'abc' };
  const response = client.invoke('voting.Voting/Vote', data);

  check(response, {
    'status is OK': (r) => r && r.status === grpc.StatusOK,
  });

  console.log(JSON.stringify(response.message));

  client.close();
  sleep(1);
};
