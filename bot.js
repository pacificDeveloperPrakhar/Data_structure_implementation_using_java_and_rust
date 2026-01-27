const fs = require("fs");
let input=""
const label=process.env.LABEL || false;

let text=""
process.stdin.on("data", chunk => {
  input += chunk.toString();
});

process.stdin.on("end", () => {
  const files = input
    .trim()
    .split("\n")
    .filter(Boolean);

  console.log("📂 Files received:", files.length);
  update_readme(label,files.length)
});

//find . -type f \( -name "*.rs" -o -name "*.java" \) | LABEL="total_files" node ./bot.js"


function update_readme(label,input)
{


fs.readFile("./README_temp", (err, buffer) => {
  if (err) {
    console.error(err);
    return;
  }

  
  if (!text)
    {
    text=buffer;
    text=text.toString();
  }

  text=text.replace(`{{${label}}}`,input);
  // console.log(text)
  console.log(text)
  fs.writeFileSync("./README.md", text);
  
});

}

const { spawn } = require("child_process");

function runCommand(label, command) {
  return new Promise((resolve, reject) => {
    const cmd = spawn("sh", ["-c", command]);

    

    cmd.stdout.on("data", data => {
      console.log(label)
      update_readme(label,data.toString());
      
    });

    cmd.stderr.on("data", data => {
      console.error(`[${label} error]`, data.toString());
    });

    cmd.on("close", code => {
      if (code !== 0) {
        reject(new Error(`${label} exited with code ${code}`));
      } else {
        resolve("shell did not close properly");
      }
    });
  });
}

async function run_script() {
  try {
    const javaCmd =
      'find . -type f -name "*.java" | wc -l';

    const rustCmd =
      'find . -type f -name "*.rs" | wc -l';

    const totalCmd =
      'find . -type f \\( -name "*.java" -o -name "*.rs" \\) | wc -l';

    const javaCommittedCmd =
      'git log --since="today" --name-only --pretty="" | grep "\\.java$" | sort -u | wc -l';

    const rustCommittedCmd =
      'git log --since="today" --name-only --pretty="" | grep "\\.rs$" | sort -u | wc -l';

    const javaFiles = await runCommand("total_files_java", javaCmd);
    const rustFiles = await runCommand("total_files_rust", rustCmd);
    const totalFiles = await runCommand("total_files", totalCmd);

    const javaCommitted = await runCommand(
      "total_commit_java",
      javaCommittedCmd
    );

    const rustCommitted = await runCommand(
      "total_commit_rust",
      rustCommittedCmd
    );

    console.log("Total Java files :", javaFiles);
    console.log("Total Rust files :", rustFiles);
    console.log("Total files      :", totalFiles);
    console.log("Java committed   :", javaCommitted);
    console.log("Rust committed   :", rustCommitted);

  } catch (err) {
    console.error(err.message);
  }
}

run_script();
