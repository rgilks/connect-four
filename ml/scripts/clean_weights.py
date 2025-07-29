#!/usr/bin/env python3
"""
Clean model weights by removing training_history and saving it separately
This makes model files much smaller for frontend use while preserving training data
"""

import json
import argparse
import sys
from pathlib import Path
from typing import Dict, Any, Optional
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


class WeightCleaner:
    def __init__(self):
        self.weights_dir = Path.cwd() / "ml" / "data" / "weights"
        self.weights_dir.mkdir(parents=True, exist_ok=True)

    def clean_weights(
        self,
        input_file: str,
        output_file: Optional[str] = None,
        save_history: bool = True,
    ) -> bool:
        """Clean weights by removing training_history and optionally saving it separately"""

        input_path = Path(input_file)
        if not input_path.exists():
            logger.error(f"❌ Input file not found: {input_path}")
            return False

        # Determine output file
        if output_file is None:
            output_path = (
                input_path.parent / f"{input_path.stem}_clean{input_path.suffix}"
            )
        else:
            output_path = Path(output_file)

        logger.info(f"🧹 Cleaning weights from {input_path}")
        logger.info(f"📁 Output: {output_path}")

        try:
            # Load the model file
            with open(input_path, "r") as f:
                model_data = json.load(f)

            # Extract training history if it exists
            training_history = None
            if (
                "metadata" in model_data
                and "training_history" in model_data["metadata"]
            ):
                training_history = model_data["metadata"]["training_history"]
                logger.info(
                    f"📊 Found training history with {len(training_history.get('value_loss', []))} epochs"
                )

                # Remove training_history from metadata
                del model_data["metadata"]["training_history"]
                logger.info("🗑️  Removed training_history from model metadata")

            # Save cleaned model
            with open(output_path, "w") as f:
                json.dump(model_data, f, indent=2)

            # Get file sizes
            original_size = input_path.stat().st_size
            cleaned_size = output_path.stat().st_size
            size_reduction = (original_size - cleaned_size) / original_size * 100

            logger.info(f"✅ Cleaned model saved: {output_path}")
            logger.info(
                f"📏 Size reduction: {original_size / 1024 / 1024:.1f}MB → {cleaned_size / 1024 / 1024:.1f}MB ({size_reduction:.1f}% smaller)"
            )

            # Save training history separately if requested
            if save_history and training_history:
                history_file = (
                    output_path.parent / f"{output_path.stem}_training_history.json"
                )
                with open(history_file, "w") as f:
                    json.dump(
                        {
                            "metadata": {
                                "source_model": input_path.name,
                                "cleaned_at": Path(__file__).name,
                                "epochs": len(training_history.get("value_loss", [])),
                            },
                            "training_history": training_history,
                        },
                        f,
                        indent=2,
                    )
                logger.info(f"📊 Training history saved separately: {history_file}")

            return True

        except Exception as e:
            logger.error(f"❌ Error cleaning weights: {e}")
            return False

    def clean_all_weights(self, pattern: str = "ml_ai_weights*.json") -> bool:
        """Clean all weight files matching the pattern"""
        weight_files = list(self.weights_dir.glob(pattern))

        if not weight_files:
            logger.warning(f"⚠️  No files found matching pattern: {pattern}")
            return False

        logger.info(f"🧹 Found {len(weight_files)} weight files to clean")

        success_count = 0
        for weight_file in weight_files:
            if self.clean_weights(str(weight_file)):
                success_count += 1

        logger.info(
            f"✅ Successfully cleaned {success_count}/{len(weight_files)} files"
        )
        return success_count == len(weight_files)

    def copy_clean_to_public(self, clean_file: str) -> bool:
        """Copy cleaned model to public directory for frontend use"""
        clean_path = Path(clean_file)
        if not clean_path.exists():
            logger.error(f"❌ Cleaned file not found: {clean_path}")
            return False

        public_dir = Path.cwd() / "public" / "ml" / "data" / "weights"
        public_dir.mkdir(parents=True, exist_ok=True)

        public_path = public_dir / clean_path.name

        try:
            import shutil

            shutil.copy2(clean_path, public_path)
            logger.info(f"🌐 Copied to public directory: {public_path}")
            return True
        except Exception as e:
            logger.error(f"❌ Error copying to public: {e}")
            return False


def main():
    parser = argparse.ArgumentParser(
        description="Clean model weights by removing training_history"
    )
    parser.add_argument(
        "input_file", nargs="?", help="Input weights file (or use --all)"
    )
    parser.add_argument("--output", help="Output file name")
    parser.add_argument(
        "--all", action="store_true", help="Clean all ml_ai_weights*.json files"
    )
    parser.add_argument(
        "--pattern", default="ml_ai_weights*.json", help="File pattern for --all"
    )
    parser.add_argument(
        "--no-history",
        action="store_true",
        help="Don't save training history separately",
    )
    parser.add_argument(
        "--copy-to-public",
        action="store_true",
        help="Copy cleaned file to public directory",
    )

    args = parser.parse_args()

    cleaner = WeightCleaner()

    if args.all:
        success = cleaner.clean_all_weights(args.pattern)
    elif args.input_file:
        success = cleaner.clean_weights(
            args.input_file, args.output, save_history=not args.no_history
        )

        if success and args.copy_to_public:
            output_file = (
                args.output
                or f"{Path(args.input_file).stem}_clean{Path(args.input_file).suffix}"
            )
            cleaner.copy_clean_to_public(output_file)
    else:
        parser.print_help()
        return

    if success:
        logger.info("🎉 Weight cleaning completed successfully!")
        sys.exit(0)
    else:
        logger.error("❌ Weight cleaning failed!")
        sys.exit(1)


if __name__ == "__main__":
    main()
