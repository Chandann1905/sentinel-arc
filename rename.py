import os
import glob

# Search parameters
replacements = {
    "project_brain": "sentinel_arc",
    "project-brain": "sentinel-arc",
    "Project Brain": "Sentinel Arc",
    "project_brain_core": "sentinel_arc_core",
    "project-brain-core": "sentinel-arc-core",
    "project_brain_knowledge": "sentinel_arc_knowledge",
    "project-brain-knowledge": "sentinel-arc-knowledge",
}

def is_text_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            f.read(1024)
        return True
    except UnicodeDecodeError:
        return False

# Directories and files to check
extensions = ["*.rs", "*.toml", "*.md", "*.sql", "*.yml"]

def scan_and_replace(directory):
    for root, dirs, files in os.walk(directory):
        if "target" in dirs: dirs.remove("target")
        if ".git" in dirs: dirs.remove(".git")
        
        for file in files:
            if not any(file.endswith(ext.replace("*", "")) for ext in extensions) and file not in ["Cargo.toml", "Cargo.lock", "LICENSE"]:
                continue
                
            filepath = os.path.join(root, file)
            if not is_text_file(filepath): continue
            
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
                
            original_content = content
            for old, new in replacements.items():
                content = content.replace(old, new)
                
            if content != original_content:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"Updated: {filepath}")

if __name__ == "__main__":
    scan_and_replace(".")
