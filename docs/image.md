# Image Processing Documentation

## Overview

The tool provides comprehensive image processing capabilities for both individual files and folders.

### Key Features

- Format conversion: PNG (with Interlacing), JPEG (Progressive), WebP, AVIF
- Image manipulation: resize, crop, quality adjustment
- Output options: file replacement, new output, clipboard
- AI integration: image generation (OpenAI key required)
- Metadata extraction: AI alt text, dominant colors, data URLs, blur effects, blurhash

## Default Behavior

Basic usage:

```bash
strawr img "project/images/dog.jpg"
```

Default settings:

- Maximum size: 1024px
- Output format: WebP
- Quality: 80%
- Action: In-place replacement (unless output specified)

## Configuration

Customize defaults via config file:

```bash
# Open config file
strawr config

# Specify config path
strawr config --path /
```

Example config.json:

```json
{
  "img": {
    "defaults": {
      "format": "jpg",
      "quality": 100,
      "progressive": true
    }
  }
}
```

## Core Arguments

### Format (`--format`, `-f`)

```bash
# Convert folder contents
strawr img "project/images" -f "jpeg"

# Convert single file with output
strawr img "project/images/dog.jpg" -o "/newImages" --format "webp"
```

Supported: jpg/jpeg, png, webp, avif

### Output Path (`--output`, `-o`)

**Folder output:**

```bash
strawr img "project/images" -o "newImages" --format "webp"
```

- Creates output folder if missing
- Adds to existing folder if present

**Single file output:**

```bash
strawr img "project/images/dog.jpg" -o "newImages/dog" --format "webp"
```

**Invalid usage:**

```bash
# Error: Cannot save folder to single file
strawr img "project/images" -o "newImages.webp"
```

### Quality (`--quality`, `-q`)

Compression quality control (0-100)

### Size (`--size`, `-s`)

Image dimension specifications

### Crop (`--crop`, `-c`)

Cropping parameters

### Additional Options

- `--clipboard`: Copy to clipboard instead of saving
- `--deleteOriginal`: Remove source files after processing
- `--discardMultipleFormats`: Handle duplicate filenames with different extensions

## Subcommands

### AI Image Generation (`gen`)

```bash
strawr img gen "A flying crocodile" -o "croc" --aspect "square"
```

Options:

- `--aspect`/`-a`: square, landscape, etc.
- Supports standard arguments: output, format, quality, size

### Metadata Retrieval (`get`)

```bash
strawr img get "project/images/dog.png" --color
```

Available data:

- `--info`: Full metadata
- `--compare`: Similarity comparison
- `--alt`: AI-generated alt text
- `--color`: Dominant color extraction
- `--dataUrl`: Base64 data URL
- `--blurDataUrl`: Placeholder blur URL
- `--blurHash`: Blurhash string
