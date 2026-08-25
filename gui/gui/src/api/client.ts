const API_BASE_URL = '/v1';

export class ApiClient {
  static async get(endpoint: string) {
    const res = await fetch(`${API_BASE_URL}${endpoint}`);
    if (!res.ok) throw new Error('API Request Failed');
    return res.json();
  }

  static async post(endpoint: string, data: any) {
    const res = await fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error('API Request Failed');
    return res.json();
  }
}
