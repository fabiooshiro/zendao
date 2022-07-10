const { exec } = require('child_process');

function main() {
    const deployProcess = exec(`anchor deploy`, {
        env: process.env
    })
    deployProcess.stdout.pipe(process.stdout);
    deployProcess.stdout.on('data', msg => {
        if (/There was a problem deploying/.test(msg)) {
            console.log('Vamos tentar de novo!!!')
            setTimeout(() => main(), 1000);
        }
    })
    deployProcess.stderr.on('data', err => {
        console.log({ err })
    })
    deployProcess.on('exit', code => console.log('final exit code is', code))
}

main()