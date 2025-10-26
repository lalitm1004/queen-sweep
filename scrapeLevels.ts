/**
 * This script fetches and parses level data from the open-source repository:
 * https://github.com/samimsu/queens-game-linkedin
 *
 * The original project is licensed under the MIT License.
 *
 * This script is an independent parser built to transform that data
 * into a structured format for AI agents.
 */

import axios from 'axios';
import { writeFileSync } from 'fs';

interface LevelData {
    id: number;
    size: number;
    color_regions: string[][];
}

interface GitHubFile {
    name: string;
    download_url: string;
    type: string;
}

class LevelParser {
    private readonly repo = 'samimsu/queens-game-linkedin';
    private readonly levelsPath = 'src/utils/levels';
    private readonly baseUrl = `https://api.github.com/repos/${this.repo}/contents`;

    async fetchAndParseLevels(): Promise<LevelData[]> {
        try {
            console.log('Fetching level files from GitHub...');

            // Get list of files in the levels directory
            const files = await this.getLevelFiles();
            const levelFiles = files.filter(file =>
                file.name.endsWith('.ts') && file.name !== 'level-sample.ts'
            );

            console.log(`Found ${levelFiles.length} level files (excluding level-sample.ts)`);

            // Sort files by level number to maintain order
            const sortedFiles = levelFiles.sort((a, b) => {
                const numA = this.extractLevelNumber(a.name);
                const numB = this.extractLevelNumber(b.name);
                return numA - numB;
            });

            const parsedLevels: LevelData[] = [];

            // Download and parse each file
            for (const file of sortedFiles) {
                try {
                    const levelNumber = this.extractLevelNumber(file.name);
                    const levelData = await this.downloadAndParseFile(file, levelNumber);
                    parsedLevels.push(levelData);
                    console.log(`✓ Parsed ${file.name} -> ID: ${levelNumber}`);
                } catch (error) {
                    console.error(`✗ Failed to parse ${file.name}:`, error instanceof Error ? error.message : error);
                }
            }

            return parsedLevels;

        } catch (error) {
            throw new Error(`Failed to fetch levels: ${error instanceof Error ? error.message : error}`);
        }
    }

    private extractLevelNumber(filename: string): number {
        const match = filename.match(/level(\d+)\.ts/);
        if (!match) {
            throw new Error(`Could not extract level number from filename: ${filename}`);
        }
        return parseInt(match[1]);
    }

    private async getLevelFiles(): Promise<GitHubFile[]> {
        const response = await axios.get<GitHubFile[]>(`${this.baseUrl}/${this.levelsPath}`, {
            headers: {
                'User-Agent': 'LevelParser',
                'Accept': 'application/vnd.github.v3+json'
            }
        });

        return response.data;
    }

    private async downloadAndParseFile(file: GitHubFile, id: number): Promise<LevelData> {
        const response = await axios.get<string>(file.download_url);
        return this.parseLevelContent(response.data, id);
    }

    private parseLevelContent(content: string, id: number): LevelData {
        // Extract size using regex
        const sizeMatch = content.match(/size:\s*(\d+)/);
        if (!sizeMatch) {
            throw new Error('Could not find size in level file');
        }
        const size = parseInt(sizeMatch[1]);

        // Extract colorRegions array - try multiple patterns
        let regionsMatch = content.match(/colorRegions:\s*(\[.*?\])\s*,?\s*regionColors/s);
        if (!regionsMatch) {
            // Alternative pattern in case the first one fails
            regionsMatch = content.match(/colorRegions:\s*(\[[\s\S]*?\])/);
        }

        if (!regionsMatch) {
            throw new Error('Could not find colorRegions in level file');
        }

        const color_regions = this.parseColorRegions(regionsMatch[1]);

        // Validate that the board matches the size
        if (color_regions.length !== size || color_regions[0]?.length !== size) {
            console.warn(`Warning: Board size doesn't match declared size (${color_regions.length}x${color_regions[0]?.length} vs ${size}x${size})`);
        }

        return { id, size, color_regions };
    }

    private parseColorRegions(regionsString: string): string[][] {
        try {
            // Clean up the string for parsing
            let cleanedString = regionsString
                .replace(/'/g, '"')  // Convert single quotes to double quotes
                .replace(/,\s*]/g, ']')  // Remove trailing commas in arrays
                .replace(/,\s*}/g, '}')  // Remove trailing commas in objects
                .replace(/\s+/g, ' ')  // Normalize whitespace
                .trim();

            // Use JSON.parse for safe evaluation
            return JSON.parse(cleanedString) as string[][];

        } catch (error) {
            // Fallback: manual parsing for edge cases
            return this.manualParseRegions(regionsString);
        }
    }

    private manualParseRegions(regionsString: string): string[][] {
        const rows: string[][] = [];
        const rowRegex = /\[([^\]]+)\]/g;
        let rowMatch;

        while ((rowMatch = rowRegex.exec(regionsString)) !== null) {
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
            throw new Error('Could not parse color regions array');
        }

        return rows;
    }

    saveToJSONL(levels: LevelData[], outputPath: string = 'data/levels.jsonl'): void {
        const lines = levels.map(level => JSON.stringify(level));
        writeFileSync(outputPath, lines.join('\n'));
        console.log(`\nSaved ${levels.length} levels to ${outputPath} in JSONL format`);
    }

    displaySummary(levels: LevelData[]): void {
        console.log(`\nSuccessfully parsed ${levels.length} levels:`);

        levels.forEach(level => {
            const uniqueRegions = new Set(level.color_regions.flat()).size;
            console.log(`  Level ${level.id}: ${level.size}x${level.size} board, ${uniqueRegions} unique regions`);
        });
    }
}

// Main execution
async function main() {
    try {
        const parser = new LevelParser();
        const levels = await parser.fetchAndParseLevels();

        parser.displaySummary(levels);
        parser.saveToJSONL(levels);
    } catch (error) {
        console.error('Error:', error instanceof Error ? error.message : error);
        process.exit(1);
    }
}

// Run the script
main();