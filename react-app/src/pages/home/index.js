import styles from './styles.module.css';

const Home = () => {

    return (
        <div className={styles.container}>
            <div className={styles.form__container}>
                <h1>{`Chatrooms`}</h1>

                <input className={styles.input} placeholder='username...' />

                <select className={styles.input}>
                    <option>-- Select Room --</option>
                </select>

                <button className='btn btn-secondary' style={{ width: '100%' }}>Join Room</button>
                <button className='btn btn-secondary' style={{ width: '100%' }}>Create New Room</button>
            </div>
        </div>
    )
}

export default Home;