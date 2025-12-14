#!/usr/bin/env python3
"""
Script to generate SUMMARY.md based on the directory structure under /src
"""
import os
from pathlib import Path
from typing import List, Tuple, Optional

def get_display_name(filename: str) -> str:
    """Convert filename to display name"""
    # Remove .md extension
    name = filename.replace('.md', '')
    # For README files, use the parent directory name
    if name == 'README':
        return None
    return name

def process_directory(base_path: Path, dir_path: Path, level: int = 0) -> List[str]:
    """Recursively process directory and generate SUMMARY entries"""
    lines = []
    indent = '  ' * level
    
    # Get all items in directory
    try:
        items = sorted(dir_path.iterdir())
    except PermissionError:
        return lines
    
    # Separate files and directories
    md_files = []
    subdirs = []
    
    for item in items:
        if item.is_file() and item.suffix == '.md' and item.name != 'SUMMARY.md':
            md_files.append(item)
        elif item.is_dir():
            subdirs.append(item)
    
    # Process subdirectories
    for subdir in subdirs:
        # Check if there's a README.md in the subdirectory
        readme_path = subdir / 'README.md'
        relative_path = readme_path.relative_to(base_path)
        
        # Use directory name as the title
        title = subdir.name
        
        if readme_path.exists():
            lines.append(f'{indent}- [{title}](./{relative_path})')
            # Process files in subdirectory with increased indentation
            subdir_lines = process_directory(base_path, subdir, level + 1)
            lines.extend(subdir_lines)
        else:
            # If no README, still process subdirectory
            lines.append(f'{indent}- [{title}]')
            subdir_lines = process_directory(base_path, subdir, level + 1)
            lines.extend(subdir_lines)
    
    # Process markdown files (excluding README.md as it's already processed)
    for md_file in md_files:
        if md_file.name == 'README.md':
            continue
        
        relative_path = md_file.relative_to(base_path)
        display_name = get_display_name(md_file.name)
        if display_name:
            lines.append(f'{indent}- [{display_name}](./{relative_path})')
    
    return lines

def generate_summary(src_path: Path) -> str:
    """Generate SUMMARY.md content from src directory structure"""
    lines = ['# Summary', '']
    
    # Add aboutMe.md at the top
    about_me = src_path / 'aboutMe.md'
    if about_me.exists():
        lines.append('- [about me](./aboutMe.md)')
        lines.append('')
    
    # Get all subdirectories
    subdirs = [d for d in sorted(src_path.iterdir()) if d.is_dir()]
    
    for subdir in subdirs:
        # Create section header
        section_name = subdir.name.title()
        lines.append(f'# {section_name}')
        lines.append('')
        
        # Process the subdirectory
        subdir_lines = process_directory(src_path, subdir, level=0)
        lines.extend(subdir_lines)
        lines.append('')
    
    return '\n'.join(lines)

def main() -> int:
    # Get the src directory path
    script_dir = Path(__file__).parent
    src_path = script_dir / 'src'
    
    if not src_path.exists():
        print(f"Error: {src_path} does not exist")
        return 1
    
    # Generate SUMMARY content
    summary_content = generate_summary(src_path)
    
    # Write to SUMMARY.md
    summary_path = src_path / 'SUMMARY.md'
    with open(summary_path, 'w', encoding='utf-8') as f:
        f.write(summary_content)
    
    print(f"Successfully updated {summary_path}")
    return 0

if __name__ == '__main__':
    exit(main())
