library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity hlQInterface is
    port (
        clk, rst : in std_logic;
        time_for_event : in unsigned(63 downto 0);
        time_for_deadline : in unsigned(63 downto 0);
        push_event : std_logic;
        push_deadline : in std_logic;
		a_data_in : in std_logic;
		a_en_in : in std_logic;
		b_data_in : in std_logic;
		b_en_in : in std_logic;
		ID_data_in : in signed(7 downto 0);
		ID_en_in : in std_logic;
		a_data_out : out std_logic;
		a_en_out : out std_logic;
		b_data_out : out std_logic;
		b_en_out : out std_logic;
		ID_data_out : out signed(7 downto 0);
		ID_en_out : out std_logic;
		eq_en_out : out std_logic;
		lt_en_out : out std_logic;
		le_en_out : out std_logic;
		gt_en_out : out std_logic;
		ge_en_out : out std_logic;
		neq_en_out : out std_logic;
		not_a_en_out : out std_logic;
		a_and_b_en_out : out std_logic;
		a_or_b_en_out : out std_logic;
		a_impl_b_en_out : out std_logic;
		a_equiv_b_en_out : out std_logic;
		a_xor_b_en_out : out std_logic;
		true_const_en_out : out std_logic;
		time_stream_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* - b @ { b }
--* - ID @ { ID }
--* Event-based Output Streams 
--* - eq @ { ID }
--* - lt @ { ID }
--* - le @ { ID }
--* - gt @ { ID }
--* - ge @ { ID }
--* - neq @ { ID }
--* - not_a @ { a }
--* - a_and_b @ { a, b }
--* - a_or_b @ { a, b }
--* - a_impl_b @ { a, b }
--* - a_equiv_b @ { a, b }
--* - a_xor_b @ { a, b }
--* - true_const @ { ID }
--* Periodic Output Streams 
--* - time_stream @ 1Hz
--* Resulting Deadline Array
--* || time_stream ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : std_logic;
	signal a_en_push: std_logic;
	signal b_data_push : std_logic;
	signal b_en_push: std_logic;
	signal ID_data_push : signed(7 downto 0);
	signal ID_en_push: std_logic;
	signal eq_en_push : std_logic;
	signal lt_en_push : std_logic;
	signal le_en_push : std_logic;
	signal gt_en_push : std_logic;
	signal ge_en_push : std_logic;
	signal neq_en_push : std_logic;
	signal not_a_en_push : std_logic;
	signal a_and_b_en_push : std_logic;
	signal a_or_b_en_push : std_logic;
	signal a_impl_b_en_push : std_logic;
	signal a_equiv_b_en_push : std_logic;
	signal a_xor_b_en_push : std_logic;
	signal true_const_en_push : std_logic;
	signal time_stream_en_push : std_logic;
	signal time_stream_en_array : bit_array(0 downto 0);
    signal last_deadline_id : integer;
    signal time_to_queue : unsigned(63 downto 0);

begin

    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            clock_state_machine <= 0;
            time_to_queue <= (others => '0');
            push_to_queue <= '0';
            last_deadline_id <= 0;
			a_data_push <= '0';
			a_en_push <= '0';
			b_data_push <= '0';
			b_en_push <= '0';
			ID_data_push <= (others => '0');
			ID_en_push <= '0';
			eq_en_push <= '0';
			lt_en_push <= '0';
			le_en_push <= '0';
			gt_en_push <= '0';
			ge_en_push <= '0';
			neq_en_push <= '0';
			not_a_en_push <= '0';
			a_and_b_en_push <= '0';
			a_or_b_en_push <= '0';
			a_impl_b_en_push <= '0';
			a_equiv_b_en_push <= '0';
			a_xor_b_en_push <= '0';
			true_const_en_push <= '0';
			time_stream_en_push <= '0';
            -- Initialize Deadline Arrays
            --* Deadline Array
            --* || time_stream ||
			time_stream_en_array(0) <= '1';
        elsif (rising_edge(clk)) then
            clock_state_machine <= (clock_state_machine + 1) mod 4;
            if push_deadline = '1' and clock_state_machine = 0 then
                -- Deadline Handling
                push_to_queue <= '1';
                last_deadline_id <= (last_deadline_id + 1) mod 1;
                time_to_queue <= time_for_deadline;
				--* a @ { a }
				a_en_push <= '0';
				--* b @ { b }
				b_en_push <= '0';
				--* ID @ { ID }
				ID_en_push <= '0';
				--* eq @ { ID }
				eq_en_push <= '0';
				--* lt @ { ID }
				lt_en_push <= '0';
				--* le @ { ID }
				le_en_push <= '0';
				--* gt @ { ID }
				gt_en_push <= '0';
				--* ge @ { ID }
				ge_en_push <= '0';
				--* neq @ { ID }
				neq_en_push <= '0';
				--* not_a @ { a }
				not_a_en_push <= '0';
				--* a_and_b @ { a, b }
				a_and_b_en_push <= '0';
				--* a_or_b @ { a, b }
				a_or_b_en_push <= '0';
				--* a_impl_b @ { a, b }
				a_impl_b_en_push <= '0';
				--* a_equiv_b @ { a, b }
				a_equiv_b_en_push <= '0';
				--* a_xor_b @ { a, b }
				a_xor_b_en_push <= '0';
				--* true_const @ { ID }
				true_const_en_push <= '0';
				--* time_stream @ 1Hz
				time_stream_en_push <= time_stream_en_array(last_deadline_id);
            elsif push_event = '1' and clock_state_machine = 2 then
                -- Event Handling
                push_to_queue <= '1';
                time_to_queue <= time_for_event;
				--* a @ { a }
				a_data_push <= a_data_in;
				a_en_push <= a_en_in;
				--* b @ { b }
				b_data_push <= b_data_in;
				b_en_push <= b_en_in;
				--* ID @ { ID }
				ID_data_push <= ID_data_in;
				ID_en_push <= ID_en_in;
				--* eq @ { ID }
				eq_en_push <= '1' and ID_en_in;
				--* lt @ { ID }
				lt_en_push <= '1' and ID_en_in;
				--* le @ { ID }
				le_en_push <= '1' and ID_en_in;
				--* gt @ { ID }
				gt_en_push <= '1' and ID_en_in;
				--* ge @ { ID }
				ge_en_push <= '1' and ID_en_in;
				--* neq @ { ID }
				neq_en_push <= '1' and ID_en_in;
				--* not_a @ { a }
				not_a_en_push <= '1' and a_en_in;
				--* a_and_b @ { a, b }
				a_and_b_en_push <= '1' and a_en_in and b_en_in;
				--* a_or_b @ { a, b }
				a_or_b_en_push <= '1' and a_en_in and b_en_in;
				--* a_impl_b @ { a, b }
				a_impl_b_en_push <= '1' and a_en_in and b_en_in;
				--* a_equiv_b @ { a, b }
				a_equiv_b_en_push <= '1' and a_en_in and b_en_in;
				--* a_xor_b @ { a, b }
				a_xor_b_en_push <= '1' and a_en_in and b_en_in;
				--* true_const @ { ID }
				true_const_en_push <= '1' and ID_en_in;
				--* time_stream @ 1Hz
				time_stream_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				b_en_push <= '0';
				ID_en_push <= '0';
				eq_en_push <= '0';
				lt_en_push <= '0';
				le_en_push <= '0';
				gt_en_push <= '0';
				ge_en_push <= '0';
				neq_en_push <= '0';
				not_a_en_push <= '0';
				a_and_b_en_push <= '0';
				a_or_b_en_push <= '0';
				a_impl_b_en_push <= '0';
				a_equiv_b_en_push <= '0';
				a_xor_b_en_push <= '0';
				true_const_en_push <= '0';
				time_stream_en_push <= '0';
            end if;
        end if;
    end process;

    push_out <= push_to_queue;
    time_out <= time_to_queue;
	a_data_out <= a_data_push;
	a_en_out <= a_en_push;
	b_data_out <= b_data_push;
	b_en_out <= b_en_push;
	ID_data_out <= ID_data_push;
	ID_en_out <= ID_en_push;
	eq_en_out <= eq_en_push;
	lt_en_out <= lt_en_push;
	le_en_out <= le_en_push;
	gt_en_out <= gt_en_push;
	ge_en_out <= ge_en_push;
	neq_en_out <= neq_en_push;
	not_a_en_out <= not_a_en_push;
	a_and_b_en_out <= a_and_b_en_push;
	a_or_b_en_out <= a_or_b_en_push;
	a_impl_b_en_out <= a_impl_b_en_push;
	a_equiv_b_en_out <= a_equiv_b_en_push;
	a_xor_b_en_out <= a_xor_b_en_push;
	true_const_en_out <= true_const_en_push;
	time_stream_en_out <= time_stream_en_push;

end behavioral;
