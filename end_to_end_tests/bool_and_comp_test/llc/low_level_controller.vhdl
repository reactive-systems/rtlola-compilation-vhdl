library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity low_level_controller is
    port (
        clk, eclk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a : in std_logic;
		a_en : in std_logic;
		b : in std_logic;
		b_en : in std_logic;
		ID : in signed(7 downto 0);
		ID_en : in std_logic;
		eq_en : in std_logic;
		lt_en : in std_logic;
		le_en : in std_logic;
		gt_en : in std_logic;
		ge_en : in std_logic;
		neq_en : in std_logic;
		not_a_en : in std_logic;
		a_and_b_en : in std_logic;
		a_or_b_en : in std_logic;
		a_impl_b_en : in std_logic;
		a_equiv_b_en : in std_logic;
		a_xor_b_en : in std_logic;
		true_const_en : in std_logic;
		time_stream_en : in std_logic;
		data_available : in std_logic;
		eq : out std_logic;
		lt : out std_logic;
		le : out std_logic;
		gt : out std_logic;
		ge : out std_logic;
		neq : out std_logic;
		not_a : out std_logic;
		a_and_b : out std_logic;
		a_or_b : out std_logic;
		a_impl_b : out std_logic;
		a_equiv_b : out std_logic;
		a_xor_b : out std_logic;
		true_const : out std_logic;
		time_stream : out signed(7 downto 0);
		pop : out std_logic;
		eval_done : out std_logic
    );
end low_level_controller;

architecture mixed of low_level_controller is

	-- component declaration
	component evaluator is
		port (
			clk, input_clk, rst : in std_logic;
			input_time : in unsigned(63 downto 0);
			a : in std_logic;
			a_en : in std_logic;
			b : in std_logic;
			b_en : in std_logic;
			ID : in signed(7 downto 0);
			ID_en : in std_logic;
			eq_en : in std_logic;
			lt_en : in std_logic;
			le_en : in std_logic;
			gt_en : in std_logic;
			ge_en : in std_logic;
			neq_en : in std_logic;
			not_a_en : in std_logic;
			a_and_b_en : in std_logic;
			a_or_b_en : in std_logic;
			a_impl_b_en : in std_logic;
			a_equiv_b_en : in std_logic;
			a_xor_b_en : in std_logic;
			true_const_en : in std_logic;
			time_stream_en : in std_logic;
			eq : out std_logic;
			lt : out std_logic;
			le : out std_logic;
			gt : out std_logic;
			ge : out std_logic;
			neq : out std_logic;
			not_a : out std_logic;
			a_and_b : out std_logic;
			a_or_b : out std_logic;
			a_impl_b : out std_logic;
			a_equiv_b : out std_logic;
			a_xor_b : out std_logic;
			true_const : out std_logic;
			time_stream : out signed(7 downto 0);
			done : out std_logic;
			valid : out std_logic
		);
	end component;

	-- signal declaration
	signal input_clk : std_logic;
	signal current_state : integer;
	signal evaluator_done : std_logic;
	signal evaluator_valid : std_logic;
	signal pop_data : std_logic;

begin
    -- component instantiation
    evaluator_instance: evaluator
        port map (
			clk => clk,
			input_clk => input_clk,
			rst => rst,
			input_time => time_in,
			a => a,
			a_en => a_en,
			b => b,
			b_en => b_en,
			ID => ID,
			ID_en => ID_en,
			eq_en => eq_en,
			lt_en => lt_en,
			le_en => le_en,
			gt_en => gt_en,
			ge_en => ge_en,
			neq_en => neq_en,
			not_a_en => not_a_en,
			a_and_b_en => a_and_b_en,
			a_or_b_en => a_or_b_en,
			a_impl_b_en => a_impl_b_en,
			a_equiv_b_en => a_equiv_b_en,
			a_xor_b_en => a_xor_b_en,
			true_const_en => true_const_en,
			time_stream_en => time_stream_en,
			eq => eq,
			lt => lt,
			le => le,
			gt => gt,
			ge => ge,
			neq => neq,
			not_a => not_a,
			a_and_b => a_and_b,
			a_or_b => a_or_b,
			a_impl_b => a_impl_b,
			a_equiv_b => a_equiv_b,
			a_xor_b => a_xor_b,
			true_const => true_const,
			time_stream => time_stream,
			done => evaluator_done,
			valid => evaluator_valid
        );

    process(eclk, rst) begin
		if rst='1' then
			input_clk <= '0';
			current_state <= 0;
			pop_data <= '0';
		elsif rising_edge(eclk) then
            if (current_state = 0 and data_available = '1') then
                -- idle
                pop_data <= '1';
                input_clk <= '0';
                current_state <= 1;
            elsif current_state = 1 then
                -- pop
                input_clk <= '1';
                pop_data <= '0';
                current_state <= 2;
            elsif current_state = 2 and evaluator_done = '1' then
                -- evaluate_done
                if data_available = '1' then
                    pop_data <= '1';
                    input_clk <= '0';
                    current_state <= 1;
                else
                    input_clk <= '0';
                    current_state <= 0;
                end if;
            end if;
        end if;
	end process;

	pop <= pop_data;
	eval_done <= input_clk;

end mixed;