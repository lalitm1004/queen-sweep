/**
 * This script fetches and parses level data from the open-source repository:
 * https://github.com/samimsu/queens-game-linkedin
 *
 * The original project is licensed under the MIT License.
 *
 * This script is an independent parser built to transform that data
 * into a structured format for my AI agent.
 */

import axios from 'axios';
import { writeFileSync, mkdirSync, existsSync } from 'fs';
import { dirname } from 'path';

interface LevelData {
    id: number;
    size: number;
    regions: number[][];
    source: string;
}

interface GitHubFile {
    name: string;
    download_url: string;
    type: string;
}

class LevelParser {
    private readonly repo = 'samimsu/queens-game-linkedin';
    private readonly baseUrl = `https://api.github.com/repos/${this.repo}/contents`;

    private readonly sources = [
        { name: 'base-levels', path: 'src/utils/levels' },
        { name: 'community-levels', path: 'src/utils/community-levels' },
        { name: 'bonus-levels', path: 'src/utils/bonus-levels' }
    ];

    async processAll(): Promise<void> {
        for (const src of this.sources) {
            const levels = await this.fetchAndParseLevels(src.path, src.name);
            this.displaySummary(levels, src.name);
            this.saveToJSONL(levels, `data/${src.name}.jsonl`);
        }
    }

    async fetchAndParseLevels(path: string, source: string): Promise<LevelData[]> {
        try {
            console.log(`\nFetching ${source} from: ${path}`);

            const files = await this.getFiles(path);

            const levelFiles = files.filter(f => {
                const name = f.name;

                if (name === 'levelSample.ts') return false;
                if (name === 'level-sample.ts') return false;

                return name.endsWith('.ts');
            });

            const sorted = levelFiles.sort((a, b) => a.name.localeCompare(b.name));

            const parsed: LevelData[] = [];
            let bonusId = 1;

            for (const file of sorted) {
                try {
                    let id;

                    if (source === 'bonus-levels') {
                        id = bonusId++;
                    } else {
                        id = this.extractLevelNumber(file.name);
                    }

                    const data = await this.downloadAndParse(file, id, source);
                    parsed.push(data);

                    console.log(`✓ Parsed ${file.name} → ID: ${id}`);
                } catch (err) {
                    console.error(`✗ Failed to parse ${file.name}:`, err);
                }
            }

            return parsed;

        } catch (err) {
            throw new Error(`Failed to fetch ${source}: ${err}`);
        }
    }


    private extractLevelNumber(filename: string): number {
        const match = filename.match(/level(\d+)\.ts/);
        if (!match) {
            throw new Error(`No level number found in ${filename}`);
        }
        return parseInt(match[1]);
    }

    private async getFiles(path: string): Promise<GitHubFile[]> {
        const url = `${this.baseUrl}/${path}`;
        const res = await axios.get<GitHubFile[]>(url, {
            headers: {
                'User-Agent': 'LevelParser',
                'Accept': 'application/vnd.github.v3+json'
            }
        });

        return res.data;
    }

    private async downloadAndParse(file: GitHubFile, id: number, source: string): Promise<LevelData> {
        const res = await axios.get<string>(file.download_url);
        return this.parseLevelContent(res.data, id, source);
    }

    private parseLevelContent(content: string, id: number, source: string): LevelData {
        const sizeMatch = content.match(/size:\s*(\d+)/);
        if (!sizeMatch) {
            throw new Error('Missing size');
        }
        const size = parseInt(sizeMatch[1]);

        let regionsMatch = content.match(/colorRegions:\s*(\[.*?\])\s*,?\s*regionColors/s);

        if (!regionsMatch) {
            regionsMatch = content.match(/colorRegions:\s*(\[[\s\S]*?\])/);
        }

        if (!regionsMatch) {
            throw new Error('Missing colorRegions');
        }

        const strGrid = this.parseColorRegionsRaw(regionsMatch[1]);
        const numGrid = this.convertLettersToIds(strGrid);

        return {
            id,
            size,
            regions: numGrid,
            source
        };
    }

    private parseColorRegionsRaw(raw: string): string[][] {
        try {
            let cleaned = raw
                .replace(/'/g, '"')
                .replace(/,\s*]/g, ']')
                .replace(/,\s*}/g, '}')
                .replace(/\s+/g, ' ')
                .trim();

            return JSON.parse(cleaned);
        } catch {
            return this.manualParseRegions(raw);
        }
    }

    private manualParseRegions(raw: string): string[][] {
        const rows: string[][] = [];
        const rowRegex = /\[([^\]]+)\]/g;
        let rowMatch;

        while ((rowMatch = rowRegex.exec(raw)) !== null) {
            const rowContent = rowMatch[1];
            const cellRegex = /["']([A-Z])["']/g;
            const cells: string[] = [];
            let cellMatch;

            while ((cellMatch = cellRegex.exec(rowContent)) !== null) {
                cells.push(cellMatch[1]);
            }

            if (cells.length > 0) {
                rows.push(cells);
            }
        }

        if (rows.length === 0) {
            throw new Error('Could not manually parse regions');
        }

        return rows;
    }

    private convertLettersToIds(grid: string[][]): number[][] {
        const map = new Map<string, number>();
        let nextId = 0;

        return grid.map(row =>
            row.map(letter => {
                if (!map.has(letter)) {
                    map.set(letter, nextId++);
                }
                return map.get(letter)!;
            })
        );
    }

    saveToJSONL(levels: LevelData[], outputPath: string): void {
        const dir = dirname(outputPath);

        if (!existsSync(dir)) {
            mkdirSync(dir, { recursive: true });
        }

        levels.sort((a, b) => a.id - b.id);

        const lines = levels.map(l => JSON.stringify(l));
        writeFileSync(outputPath, lines.join('\n'));

        console.log(`Saved ${levels.length} levels -> ${outputPath}`);
    }

    displaySummary(levels: LevelData[], source: string): void {
        console.log(`\nSummary for ${source}:`);
        for (const lvl of levels) {
            const uniq = new Set(lvl.regions.flat()).size;
            console.log(
                `  Level ${lvl.id}: ${lvl.size}x${lvl.size}, ${uniq} region IDs`
            );
        }
    }
}

async function main() {
    try {
        const parser = new LevelParser();
        await parser.processAll();
    } catch (err) {
        console.error(err);
        process.exit(1);
    }
}

main();
