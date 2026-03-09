const { init, runCallbacks, callback, SteamCallback } = require('../index.js')

const client = init(480);

async function testGlobalStats() {
    console.log('Testing global stats...');

    // Register callback for GlobalStatsReceived
    const handle = client.callback.register(SteamCallback.GlobalStatsReceived, (value) => {
        console.log('GlobalStatsReceived callback:', value);
    });

    try {
        // Test requestGlobalStats
        console.log('Requesting global stats with 7 days history...');
        const gameId = await client.stats.requestGlobalStats(7);
        console.log('Global stats received for game:', gameId);

        // Get stat name from environment variable or use default
        const statName = process.env.TEST_GLOBAL_STAT_NAME || 'test_stat';
        console.log('Using stat name:', statName);

        // Test getGlobalInt
        const intValue = client.stats.getGlobalInt(statName);
        console.log('Global int value:', intValue);

        // Test getGlobalFloat
        const floatValue = client.stats.getGlobalFloat(statName);
        console.log('Global float value:', floatValue);

        // Test getGlobalIntHistory
        const intHistory = client.stats.getGlobalIntHistory(statName, 7);
        console.log('Global int history:', intHistory);

        // Test getGlobalFloatHistory
        const floatHistory = client.stats.getGlobalFloatHistory(statName, 7);
        console.log('Global float history:', floatHistory);

        console.log('All global stats tests completed successfully!');
    } catch (error) {
        console.error('Error during global stats test:', error);
    } finally {
        // Clean up callback handle
        handle.disconnect();
    }
}

// Run tests
testGlobalStats().then(() => {
    console.log('Stats test completed');
    process.exit(0);
}).catch((error) => {
    console.error('Stats test failed:', error);
    process.exit(1);
});
