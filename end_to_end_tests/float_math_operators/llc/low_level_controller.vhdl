library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity low_level_controller is
    port (
        clk, eclk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a : in sfixed(8 downto -23);
		a_en : in std_logic;
		b : in sfixed(8 downto -23);
		b_en : in std_logic;
		c : in sfixed(8 downto -23);
		c_en : in std_logic;
		plus_op_en : in std_logic;
		minus_op_en : in std_logic;
		mult_op_en : in std_logic;
		func_abs_en : in std_logic;
		func_sqrt_en : in std_logic;
		counter_en : in std_logic;
		data_available : in std_logic;
		plus_op : out sfixed(8 downto -23);
		minus_op : out sfixed(8 downto -23);
		mult_op : out sfixed(8 downto -23);
		func_abs : out sfixed(8 downto -23);
		func_sqrt : out sfixed(8 downto -23);
		counter : out signed(31 downto 0);
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
			a : in sfixed(8 downto -23);
			a_en : in std_logic;
			b : in sfixed(8 downto -23);
			b_en : in std_logic;
			c : in sfixed(8 downto -23);
			c_en : in std_logic;
			plus_op_en : in std_logic;
			minus_op_en : in std_logic;
			mult_op_en : in std_logic;
			func_abs_en : in std_logic;
			func_sqrt_en : in std_logic;
			counter_en : in std_logic;
			plus_op : out sfixed(8 downto -23);
			minus_op : out sfixed(8 downto -23);
			mult_op : out sfixed(8 downto -23);
			func_abs : out sfixed(8 downto -23);
			func_sqrt : out sfixed(8 downto -23);
			counter : out signed(31 downto 0);
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
			c => c,
			c_en => c_en,
			plus_op_en => plus_op_en,
			minus_op_en => minus_op_en,
			mult_op_en => mult_op_en,
			func_abs_en => func_abs_en,
			func_sqrt_en => func_sqrt_en,
			counter_en => counter_en,
			plus_op => plus_op,
			minus_op => minus_op,
			mult_op => mult_op,
			func_abs => func_abs,
			func_sqrt => func_sqrt,
			counter => counter,
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