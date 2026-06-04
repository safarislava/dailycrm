import http from 'k6/http';
import { check, sleep } from 'k6';

const BASE = 'http://localhost:8080/api';

export const options = {
    stages: [
        { duration: '30s', target: 10 },
        { duration: '2m',  target: 50 },
        { duration: '1m',  target: 100 },
        { duration: '30s', target: 0 },
    ],
    thresholds: {
        'http_req_duration{type:api}': ['p(95)<500'],
        http_req_failed:               ['rate<0.01'],
    },
};

export function setup() {
    const res = http.post(`${BASE}/auth/login`, JSON.stringify({ username: 'admin', password: 'admin123' }), {
        headers: { 'Content-Type': 'application/json' },
    });
    if (res.status !== 200) {
        throw new Error(`Login failed: ${res.status} ${res.body}`);
    }
    return { token: res.json('access_token') };
}

export default function (data) {
    const headers = {
        'Authorization': `Bearer ${data.token}`,
        'Content-Type': 'application/json',
    };

    let res = http.get(`${BASE}/projects`, { headers, tags: { type: 'api' } });
    check(res, { 'get projects': (r) => r.status === 200 });
    if (res.status !== 200) { sleep(1); return; }

    const projects = res.json();
    if (!projects?.length) { sleep(1); return; }

    const projectId = projects[0].id;

    res = http.get(`${BASE}/projects/${projectId}/stages`, { headers, tags: { type: 'api' } });
    check(res, { 'get stages': (r) => r.status === 200 });
    if (res.status !== 200) { sleep(1); return; }

    const stages = res.json();
    if (stages?.length) {
        const stagePosition = stages[0].position;

        res = http.get(`${BASE}/projects/${projectId}/stages/${stagePosition}/comments`, { headers, tags: { type: 'api' } });
        check(res, { 'get comments': (r) => r.status === 200 });

        res = http.post(
            `${BASE}/projects/${projectId}/stages/${stagePosition}/comments`,
            JSON.stringify({ text: 'Load test comment' }),
            { headers, tags: { type: 'api' } }
        );
        check(res, { 'create comment': (r) => r.status === 201 });
    }

    sleep(1);
}