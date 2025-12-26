import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.ticker import ScalarFormatter
from pathlib import Path
from dataclasses import dataclass
from enum import StrEnum
from typing import Callable, Final, List, Tuple


BASE_DIRECTORY: Final[Path] = Path(__file__).parent
STATS_DIRECTORY: Final[Path] = BASE_DIRECTORY / "stats"
PLOTS_DIRECTORY: Final[Path] = STATS_DIRECTORY / "plots"

COLORS: Final[dict[str, str]] = {
    "avg": "#6366F1",
    "p90": "#10B981",
    "p99": "#F59E0B",
}


class Category(StrEnum):
    BASE = "base"
    BONUS = "bonus"


class Heuristic(StrEnum):
    NONE = "no-heuristic"
    SMALLEST_REGION_BY_EMPTY_CELLS = "smallest-region-by-empty-cells"
    SMALLEST_REGION_FIRST = "smallest-region-first"

    @property
    def display_label(self) -> str:
        match self:
            case Heuristic.NONE:
                return "No Heuristic"
            case Heuristic.SMALLEST_REGION_BY_EMPTY_CELLS:
                return "Smallest Region (Empty Cells)"
            case Heuristic.SMALLEST_REGION_FIRST:
                return "Smallest Region First"


@dataclass
class Stats:
    avg: float
    p90: float
    p99: float


@dataclass
class BenchmarkStats:
    category: Category
    heuristic: Heuristic
    latency: Stats
    steps: Stats


def load_csv(
    stats_directory: Path, category: Category, heuristic: Heuristic
) -> pd.DataFrame:
    path = stats_directory / f"{category.value}_{heuristic.value}.csv"
    return pd.read_csv(path)


def compute_stats(df: pd.DataFrame, column: str) -> Stats:
    return Stats(
        avg=df[column].mean(),
        p90=df[column].quantile(0.90),
        p99=df[column].quantile(0.99),
    )


def load_all_stats(stats_directory: Path) -> List[BenchmarkStats]:
    results: List[BenchmarkStats] = []

    for category in Category:
        for heuristic in Heuristic:
            df = load_csv(stats_directory, category, heuristic)

            latency_stats = compute_stats(df, "duration_ns")
            latency_stats.avg /= 1e6
            latency_stats.p90 /= 1e6
            latency_stats.p99 /= 1e6

            steps_stats = compute_stats(df, "steps_taken")

            results.append(
                BenchmarkStats(
                    category=category,
                    heuristic=heuristic,
                    latency=latency_stats,
                    steps=steps_stats,
                )
            )

    return results


def filter_by_category(
    stats: List[BenchmarkStats], category: Category
) -> List[BenchmarkStats]:
    return [s for s in stats if s.category == category]


def create_bar_chart(
    stats: List[BenchmarkStats],
    title: str,
    ylabel: str,
    get_values: Callable[[BenchmarkStats], Tuple[float, float, float]],
    output_path: Path,
    use_log_scale: bool = False,
) -> None:
    labels = [s.heuristic.display_label for s in stats]
    avg_values = [get_values(s)[0] for s in stats]
    p90_values = [get_values(s)[1] for s in stats]
    p99_values = [get_values(s)[2] for s in stats]

    x = range(len(labels))
    width = 0.25

    fig, ax = plt.subplots(figsize=(12, 7))

    ax.bar(
        [i - width for i in x],
        avg_values,
        width,
        label="Average",
        color=COLORS["avg"],
        edgecolor="white",
        linewidth=1.2,
    )
    ax.bar(
        x,
        p90_values,
        width,
        label="P90",
        color=COLORS["p90"],
        edgecolor="white",
        linewidth=1.2,
    )
    ax.bar(
        [i + width for i in x],
        p99_values,
        width,
        label="P99",
        color=COLORS["p99"],
        edgecolor="white",
        linewidth=1.2,
    )

    ax.set_xlabel("Heuristic", fontsize=12, fontweight="bold")
    ax.set_ylabel(ylabel, fontsize=12, fontweight="bold")
    ax.set_title(title, fontsize=14, fontweight="bold", pad=20)

    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=0, ha="center", fontsize=10)

    ax.legend(
        loc="upper left",
        frameon=True,
        shadow=True,
        fancybox=True,
        fontsize=10,
        framealpha=0.95,
    )

    ax.grid(True, axis="both", alpha=0.4, linestyle="--", linewidth=0.8)
    ax.set_axisbelow(True)

    if use_log_scale:
        ax.set_yscale("log")
        ax.yaxis.set_major_formatter(ScalarFormatter())
        ax.ticklabel_format(axis="y", style="plain")
        ax.grid(True, which="both", axis="y", alpha=0.4, linestyle="--", linewidth=0.8)
        ax.grid(True, which="minor", axis="y", alpha=0.2, linestyle=":", linewidth=0.5)

    ax.set_facecolor("#f8f9fa")
    fig.set_facecolor("white")

    output_path.parent.mkdir(parents=True, exist_ok=True)
    plt.tight_layout()
    plt.savefig(output_path, dpi=300, bbox_inches="tight", facecolor="white")
    plt.close()


def get_latency_values(s: BenchmarkStats) -> Tuple[float, float, float]:
    return s.latency.avg, s.latency.p90, s.latency.p99


def get_steps_values(s: BenchmarkStats) -> Tuple[float, float, float]:
    return s.steps.avg, s.steps.p90, s.steps.p99


def generate_charts(
    stats_directory: Path,
    plots_directory: Path,
    all_stats: List[BenchmarkStats],
) -> None:
    for category in Category:
        category_stats = filter_by_category(all_stats, category)
        category_title = category.value.capitalize()

        create_bar_chart(
            category_stats,
            f"Category: {category_title} Latencies",
            "Latency (ms)",
            get_latency_values,
            plots_directory / f"{category.value}_latency.png",
        )

        create_bar_chart(
            category_stats,
            f"Category: {category_title} Latencies (Log Scale)",
            "Latency (ms)",
            get_latency_values,
            plots_directory / f"{category.value}_latency_log.png",
            use_log_scale=True,
        )

        create_bar_chart(
            category_stats,
            f"Category: {category_title} Steps Taken",
            "Steps",
            get_steps_values,
            plots_directory / f"{category.value}_steps.png",
        )

        create_bar_chart(
            category_stats,
            f"Category: {category_title} Steps Taken (Log Scale)",
            "Steps",
            get_steps_values,
            plots_directory / f"{category.value}_steps_log.png",
            use_log_scale=True,
        )


def print_summary(all_stats: List[BenchmarkStats]) -> None:
    for category in Category:
        print(f"Category: {category.value}")
        category_stats = filter_by_category(all_stats, category)

        for s in category_stats:
            print(f"\n{s.heuristic.display_label}:")
            print(
                f"  Latency - Avg: {s.latency.avg:.2f}ms, "
                f"P90: {s.latency.p90:.2f}ms, "
                f"P99: {s.latency.p99:.2f}ms"
            )
            print(
                f"  Steps   - Avg: {s.steps.avg:.0f}, "
                f"P90: {s.steps.p90:.0f}, "
                f"P99: {s.steps.p99:.0f}"
            )


def main() -> None:
    all_stats = load_all_stats(STATS_DIRECTORY)
    generate_charts(STATS_DIRECTORY, PLOTS_DIRECTORY, all_stats)
    print_summary(all_stats)
    print(f"\nCharts saved to {PLOTS_DIRECTORY}")


if __name__ == "__main__":
    main()
