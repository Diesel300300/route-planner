import type { Path, Way } from '../models/map';

const accepted_road_types = [
    "residential",
    "unclassified",
    "track",
    "service",
    "tertiary",
    "road",
    "secondary",
    "primary",
    "trunk",
    "primary_link",
    "trunk_link",
    "tertiary_link",
    "secondary_link",
    "highway",
]

function assignColorsPaths(paths: Path[]): Map<string,string> {
    if (!paths || paths.length === 0) {
        return new Map<string,string>();
    }
    const m = new Map<string,string>();
    paths.forEach((path, i) => {
        const hue = (i * 137.508) % 360;
        m.set(path.id, `hsl(${hue}, 60%, 50%)`);
    });
    return m;
}

function assignColorsWays(ways: Way[]): Map<string,string> {
    if (!ways || ways.length === 0) {
        return new Map<string,string>();
    }
    const m = new Map<string,string>();
    ways.forEach((way, i) => {
        const hue = (i * 137.508) % 360;
        m.set(way.id, `hsl(${hue}, 60%, 50%)`);
    });
    return m;
}

async function fetchWays() {
    try {
        const payload = {
            "tags": accepted_road_types
        }
        const res = await fetch('http://localhost:8000/ways_by_tags', {
            mode: 'cors',
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        });
        if (!res.ok) {
            throw new Error("Server error ${res.status}: ${res.statusText}");
        }
        return res.json();
    } catch (err) {
        console.error("Error loading nodes:", err);
    }
}

async function fetchPathsBfs(start: { lat: number, lon: number }, goal: { lat: number, lon: number }, distance: number, amount: number) {
    try {
        const res = await fetch('http://localhost:8000/paths_bfs', {
            mode: 'cors',
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                start_lat: start.lat,
                start_lon: start.lon,
                goal_lat: goal.lat,
                goal_lon: goal.lon,
                target_distance: distance,
                amount: amount,
            }),
        });
        if (!res.ok) {
            throw new Error("Server error ${res.status}: ${res.statusText}");
        }
        return res.json();
    } catch (err) {
        console.error("Error loading routes:", err);
    }

}


async function fetchPathsSpecialDijkstra(start: { lat: number, lon: number }, goal: { lat: number, lon: number }, distance: number, amount: number) {
    try {
        const res = await fetch('http://localhost:8000/paths_special_dijkstra', {
            mode: 'cors',
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                start_lat: start.lat,
                start_lon: start.lon,
                goal_lat: goal.lat,
                goal_lon: goal.lon,
                target_distance: distance,
                amount: amount,
            }),
        });
        if (!res.ok) {
            throw new Error("Server error ${res.status}: ${res.statusText}");
        }
        return res.json();
    } catch (err) {
        console.error("Error loading routes:", err);
    }
}

async function fetchPathsDfs(start: { lat: number, lon: number }, goal: { lat: number, lon: number }, distance: number, amount: number) {
    try {
        const res = await fetch('http://localhost:8000/paths_dfs', {
            mode: 'cors',
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                start_lat: start.lat,
                start_lon: start.lon,
                goal_lat: goal.lat,
                goal_lon: goal.lon,
                target_distance: distance,
                amount: amount,
            }),
        });
        if (!res.ok) {
            throw new Error("Server error ${res.status}: ${res.statusText}");
        }
        return res.json();
    } catch (err) {
        console.error("Error loading routes:", err);
    }
}


export {fetchPathsDfs, fetchPathsBfs, fetchPathsSpecialDijkstra, fetchWays, assignColorsPaths, assignColorsWays};


