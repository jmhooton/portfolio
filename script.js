fetch("https://api.github.com/repos/jmhooton/portfolio/commits?sha=main&per_page=1")
    .then(response => response.json())
    .then(commits => {
        const commitTime = commits[0].commit.committer.date;
        const commitDate = new Date(commitTime);

        function time() {
            const resultMinutes = Math.floor((Date.now() - commitDate.getTime()) / 60000);
            const days = Math.floor(resultMinutes / 1440);
            const hours = Math.floor((resultMinutes % 1440) / 60);
            const minutes = resultMinutes % 60;

            const lastCommit = document.getElementById("lastcommit");
            if (days > 0)
                lastCommit.textContent = "Last commit " + days + "d " + hours + "h " + minutes + "m ago "
            else if (hours > 0)
                lastCommit.textContent = "Last commit " + hours + "h " + minutes + "m ago "
            else
                lastCommit.textContent = "Last commit " + minutes + "m ago "
        }

        time();
        setInterval(time, 60000);
    });