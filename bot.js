const fs = require("fs").promises;
const { spawn } = require("child_process");

/**
 * Executes a shell command and returns the trimmed output
 */
function runCommand(command) {
  return new Promise((resolve, reject) => {
    const cmd = spawn("sh", ["-c", command]);
    let output = "";
    let errorOutput = "";

    cmd.stdout.on("data", data => output += data.toString());
    cmd.stderr.on("data", data => errorOutput += data.toString());

    cmd.on("close", code => {
      if (code !== 0) {
        reject(new Error(`Command failed with code ${code}: ${errorOutput}`));
      } else {
        resolve(output.trim());
      }
    });
  });
}

async function run_script() {
  try {
    // 1. Define all commands
    const commands = {
      total_files_java: 'find . -type f -name "*.java" | wc -l',
      total_files_rust: 'find . -type f -name "*.rs" | wc -l',
      total_files: 'find . -type f \\( -name "*.java" -o -name "*.rs" \\) | wc -l',
      total_commit_java: 'git log --since="today" --name-only --pretty="" | grep "\\.java$" | sort -u | wc -l',
      total_commit_rust: 'git log --since="today" --name-only --pretty="" | grep "\\.rs$" | sort -u | wc -l'
    };

    // 2. Run all commands in parallel
    console.log("📊 Gathering metrics...");
    const keys = Object.keys(commands);
    const results = await Promise.all(keys.map(key => runCommand(commands[key])));

    // 3. Map results back to labels
    const data = {};
    keys.forEach((key, index) => {
      data[key] = results[index];
    });

    // 4. Read the template
    let template = await fs.readFile("./README_temp", "utf8");

    // 5. Replace all placeholders
    for (const [label, value] of Object.entries(data)) {
      console.log(`✅ ${label}: ${value}`);
      const regex = new RegExp(`{{${label}}}`, "g");
      template = template.replace(regex, value);
    }

    // 6. Write the final file
    await fs.writeFile("./README.md", template);
    console.log("🚀 README.md updated successfully!");

  } catch (err) {
    console.error("❌ Error updating README:", err.message);
  }
}

run_script();